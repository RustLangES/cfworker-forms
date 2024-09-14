use forms_models::external::{External, ExternalJs, ExternalRead};
use forms_models::form::{Form, FormJs, FormRead};
use forms_models::shared::{D1EntityDelete, D1EntityRead};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Not;
use worker::Error::D1;

use forms_models::session::{Session, SessionCreate, SessionDelete, SessionJs, SessionRead};
use forms_models::D1EntityQueries;
use forms_shared::db::D1Action;
use forms_shared::{get_auth, FormsResponse, IntoResponse, WorkerHttpResponse};

use crate::admins::get_admins;
use crate::shared::{error_wrapper, needs_auth};
use crate::RouterContext;

pub fn get_device_id(req: &worker::Request) -> Option<String> {
    let site_id = req
        .headers()
        .get("cf-connecting-ip")
        .ok()
        .flatten()
        .or_else(|| req.headers().get("x-forwarded-for").ok().flatten())
        .or_else(|| req.cf().and_then(|cf| cf.city()))
        .inspect(|x| worker::console_log!("DeviceID-SiteID: {x:?}"))?;

    let user_agent = req
        .headers()
        .get("user-agent")
        .ok()
        .flatten()
        .inspect(|x| worker::console_log!("DeviceID-UserAgent: {x:?}"))?;

    worker::console_log!("DeviceID: {site_id} + {user_agent}");

    Some(site_id + &user_agent)
}

pub fn get_device_id_hash(req: &worker::Request) -> Option<String> {
    let device_id = get_device_id(req)?;

    let mut hasher = DefaultHasher::new();
    device_id.hash(&mut hasher);

    Some(hasher.finish().to_string())
}

fn get_session_token(form_id: usize, device_id: String) -> (String, String) {
    let mut hasher = DefaultHasher::new();
    device_id.hash(&mut hasher);

    let device_id_hash = hasher.finish().to_string();

    form_id.hash(&mut hasher);

    let hash = hasher.finish().to_string();

    let session = &mut [0u8; 16];
    for (i, c) in hash.bytes().take(16).enumerate() {
        // SAFETY: It can use only the first 16 bytes (`session` have a fixed 16 length)
        session[i] = c;
    }

    let token = uuid::Uuid::new_v8(*session).to_string();

    (device_id_hash, token)
}

pub async fn get(req: worker::Request, ctx: RouterContext) -> WorkerHttpResponse {
    error_wrapper(req, ctx, |mut req, ctx, db| async move {
        let external_code = get_auth(&mut req)?
            .map(|e|
                External::read_query(&db, ExternalRead {
                    external_id: None,
                    token: Some(e)
                }).first::<ExternalJs>()
            );

        let external_id = if let Some(external_code) = external_code {
            external_code.await?.map(|e| e.id)
        } else {
            None
        };

        let form_id: usize = ctx.param("form_id").unwrap().parse().unwrap();

        // Check if form exists
        let Some(form) = D1EntityRead::read_query(FormRead {
            id: form_id
        }, &db).first_into::<FormJs, Form>().await? else {
            return FormsResponse::json(404, &serde_json::json!({
                "errors": [ "Form doesn't exists" ],
                "success": false
            }));
        };

        if form.require_login && external_id.is_none() {
            return FormsResponse::json(401, &serde_json::json!({
                "errors": [ "This form requires external login" ],
                "success": false
            }));
        }

        let Some(device_id) = get_device_id(&req) else {
            return FormsResponse::json(400, &serde_json::json!({
                "errors": [ "Cannot get device id" ],
                "success": false
            }));
        };

        let (device_id_hash, token) = get_session_token(form_id, device_id);

        if form.multiple_times.not() {
            let old_session = Session::read_query(
                &db,
                SessionRead {
                    external_id,
                    device_id: Some(device_id_hash.clone()),
                    form_id: Some(form_id),
                    deleted: Some(true),
                    ..Default::default()
                },
            ).first::<SessionJs>().await?;

            if old_session.is_some() {
                return FormsResponse::json(403, &serde_json::json!({
                    "errors": [ "You already answer this" ],
                    "success": false
                }));
            }
        }

        match Session::create_query(
            &db,
            SessionCreate {
                form_id,
                external_id,
                device_id: device_id_hash,
                token: token.clone(),
            },
        )
        .try_run()
        .await
        {
            Ok(_) => {}
            // Check if fails by `UNIQUE` constraint
            Err(D1(error)) if error.cause().contains("UNIQUE") => {
                return FormsResponse::json(
                    200,
                    &serde_json::json!({
                        "errors": [ ],
                        "messages": [
                            "Your session already exists. This form is already open by another browser or client"
                        ],
                        "data": token,
                        "success": true
                    }),
                )
            }

            Err(why) => return Err(why.into_response()),
        }

        FormsResponse::json(
            201,
            &serde_json::json!({
                "errors": [ ],
                "data": token,
                "success": true
            }),
        )
    })
    .await
}

pub async fn delete(req: worker::Request, ctx: RouterContext) -> WorkerHttpResponse {
    error_wrapper(req, ctx, |mut req, _, db| async move {
        let Session { token, .. } = needs_auth(&mut req, &db, None).await?;

        D1EntityDelete::delete_query(
            SessionDelete {
                token: token.unwrap(),
            },
            &db,
        )
        .run()
        .await?;

        FormsResponse::ok("OK")
    })
    .await
}

pub async fn get_admin(req: worker::Request, ctx: RouterContext) -> WorkerHttpResponse {
    error_wrapper(req, ctx, |mut req, ctx, db| async move {
        let Some(token) = get_auth(&mut req)? else {
            return FormsResponse::json(
                403,
                &serde_json::json!({
                    "errors": [ "No external token provided" ],
                    "success": false
                }),
            );
        };

        let Some(external) = D1EntityRead::read_query(
            ExternalRead {
                token: Some(token),
                external_id: None,
            },
            &db,
        )
        .first_into::<ExternalJs, External>()
        .await?
        else {
            return FormsResponse::json(
                403,
                &serde_json::json!({
                    "errors": [ "Eeeeeeeeeeeerorrrrrrrrrrrrrr logueate bien chabon" ],
                    "success": false
                }),
            );
        };

        if !get_admins(&ctx)?.contains(&external.email) {
            return FormsResponse::json(
                403,
                &serde_json::json!({
                    "errors": [ "No admin email" ],
                    "success": false
                }),
            );
        }

        let form_id = 0usize;

        let Some(device_id) = get_device_id(&req) else {
            return FormsResponse::json(
                400,
                &serde_json::json!({
                    "errors": [ "Cannot get device id" ],
                    "success": false
                }),
            );
        };

        let (device_id_hash, token) = get_session_token(form_id, device_id);

        match Session::create_query(
            &db,
            SessionCreate {
                form_id,
                device_id: device_id_hash,
                external_id: Some(external.id),
                token: token.clone(),
            },
        )
        .try_run()
        .await
        {
            Ok(_) => {}
            // Check if fails by `UNIQUE` constraint
            Err(D1(error)) if error.cause().contains("UNIQUE") => {
                return FormsResponse::json(
                    200,
                    &serde_json::json!({
                        "errors": [ ],
                        "messages": [
                            "Your session already exists."
                        ],
                        "data": token,
                        "success": true
                    }),
                )
            }

            Err(why) => return Err(why.into_response()),
        }

        FormsResponse::json(
            201,
            &serde_json::json!({
                "errors": [ ],
                "data": token,
                "success": true
            }),
        )
    })
    .await
}
