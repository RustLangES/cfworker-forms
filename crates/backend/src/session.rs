use forms_models::external::{External, ExternalJs, ExternalRead};
use forms_models::form::{FormJs, FormRead};
use forms_models::shared::{D1EntityDelete, D1EntityRead};
use std::hash::{DefaultHasher, Hash, Hasher};
use worker::Error::D1;

use forms_models::session::{Session, SessionCreate, SessionDelete};
use forms_models::D1EntityQueries;
use forms_shared::db::D1Action;
use forms_shared::{get_auth, FormsResponse, IntoResponse, WorkerHttpResponse};

use crate::admins::get_admins;
use crate::shared::{error_wrapper, needs_auth};
use crate::RouterContext;

pub async fn get(req: worker::Request, ctx: RouterContext) -> WorkerHttpResponse {
    error_wrapper(req, ctx, |req, ctx, db| async move {
        let form_id: usize = ctx.param("form_id").unwrap().parse().unwrap();

        // Check if form exists
        let Some(_) = D1EntityRead::read_query(FormRead {
            id: form_id
        }, &db).first::<FormJs>().await? else {
            return FormsResponse::json(404, &serde_json::json!({
                "errors": [ "Form doesn't exists" ],
                "success": false
            }));
        };

        let device_id = req
            .headers()
            .get("cf-connecting-ip")
            .ok()
            .flatten()
            .or_else(|| req.headers().get("x-forwarded-for").ok().flatten())
            .or_else(|| req.cf().and_then(|cf| cf.city()))
            .or_else(|| req.headers().get("user-agent").ok().flatten())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());


        let mut hasher = DefaultHasher::new();
        device_id.hash(&mut hasher);
        form_id.hash(&mut hasher);
        let hash = hasher.finish().to_string();

        let session = &mut [0u8; 16];
        for (i, c) in hash.bytes().take(16).enumerate() {
            // SAFETY: It can use only the first 16 bytes (sessions have a fixed 16 length)
            session[i] = c;
        }

        let token = uuid::Uuid::new_v8(*session).to_string();

        match Session::create_query(
            &db,
            SessionCreate {
                form_id,
                external_id: None,
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

        D1EntityDelete::delete_query(SessionDelete { token }, &db)
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

        let device_id = req
            .headers()
            .get("cf-connecting-ip")
            .ok()
            .flatten()
            .or_else(|| req.headers().get("x-forwarded-for").ok().flatten())
            .or_else(|| req.cf().and_then(|cf| cf.city()))
            .or_else(|| req.headers().get("user-agent").ok().flatten())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let mut hasher = DefaultHasher::new();
        device_id.hash(&mut hasher);
        form_id.hash(&mut hasher);
        let hash = hasher.finish().to_string();

        let session = &mut [0u8; 16];
        for (i, c) in hash.bytes().take(16).enumerate() {
            // SAFETY: It can use only the first 16 bytes (sessions have a fixed 16 length)
            session[i] = c;
        }

        let token = uuid::Uuid::new_v8(*session).to_string();

        match Session::create_query(
            &db,
            SessionCreate {
                form_id,
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
