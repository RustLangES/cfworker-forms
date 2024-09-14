mod github;
mod shared;

use base64::Engine;
use forms_models::external::{ExternalCreate, ExternalJs, ExternalRead, ExternalUpdate};
use forms_models::shared::{D1EntityCreate, D1EntityRead, D1EntityUpdate};
use forms_shared::db::D1Action;
use forms_shared::{FormsResponse, IntoResponse, WorkerHttpResponse};
use github::GithubUser;
use oauth2::TokenResponse;
use shared::auth_callback;

use crate::shared::error_wrapper;
use crate::RouterContext;

pub async fn github(req: worker::Request, ctx: RouterContext) -> WorkerHttpResponse {
    error_wrapper(req, ctx, |req, ctx, _| async move {
        let redirect_to = req
            .url()
            .map_err(IntoResponse::into_response)?
            .query_pairs()
            .filter_map(|(name, val)| (name == "redirect_to").then_some(val.to_string()))
            .next();

        let authorize_url = github::get_authorize(&ctx, redirect_to);

        worker::Response::redirect(authorize_url).map_err(IntoResponse::into_response)
    })
    .await
}

pub async fn github_callback(req: worker::Request, ctx: RouterContext) -> WorkerHttpResponse {
    auth_callback(
        req,
        ctx,
        github::get_token,
        |_req, _ctx, db, token, redirect_to| async move {
            let external_token =
                base64::prelude::BASE64_STANDARD.encode(token.access_token().secret());

            let mut e = [0; 16];
            e.copy_from_slice(&external_token.as_bytes()[..16]);

            let external_token = uuid::Uuid::new_v8(e).to_string();

            let GithubUser {
                id: user_id,
                email: user_email,
                name: user_name,
                ..
            } = github::get_user(token).await?;

            let external_user = D1EntityRead::read_query(
                ExternalRead {
                    external_id: Some(user_id.to_string()),
                    token: None,
                },
                &db,
            )
            .first::<ExternalJs>()
            .await?;

            if let Some(external_user) = external_user {
                D1EntityUpdate::update_query(
                    ExternalUpdate {
                        id: external_user.id,
                        email: Some(user_email.unwrap_or(user_name)),
                        token: Some(external_token.clone()),
                    },
                    &db,
                )
                .run()
                .await?;
            } else {
                D1EntityCreate::create_query(
                    ExternalCreate {
                        kind: "Github".to_owned(),
                        external_id: user_id.to_string(),
                        email: user_email.unwrap_or(user_name),
                        token: external_token.clone(),
                    },
                    &db,
                )
                .run()
                .await?;
            }

            if let Some(redirect_to) = redirect_to {
                let url =
                    worker::Url::parse_with_params(&redirect_to, &[("code", &external_token)])
                        .map_err(|err| {
                            FormsResponse::text(
                                500,
                                format!("Cannot parse redirect url ({redirect_to}): {err}"),
                            )
                            .into_response()
                        })?;

                worker::console_log!("{}", url.as_str());

                return worker::Response::redirect(url).map_err(IntoResponse::into_response);
            }

            FormsResponse::json(
                200,
                &serde_json::json!({
                    "errors": [],
                    "data": external_token,
                    "success": true
                }),
            )
        },
    )
    .await
}
