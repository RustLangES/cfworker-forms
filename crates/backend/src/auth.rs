mod github;
mod shared;

use forms_shared::{FormsResponse, IntoResponse, WorkerHttpResponse};
use shared::auth_callback;

use crate::shared::error_wrapper;
use crate::RouterContext;

pub async fn github(req: worker::Request, ctx: RouterContext) -> WorkerHttpResponse {
    error_wrapper(req, ctx, |_, ctx, _| async move {
        let authorize_url = github::get_authorize(&ctx);

        worker::Response::redirect(authorize_url).map_err(IntoResponse::into_response)
    })
    .await
}

pub async fn github_callback(req: worker::Request, ctx: RouterContext) -> WorkerHttpResponse {
    auth_callback(
        req,
        ctx,
        github::get_token,
        |_req, _ctx, db, token| async move {
            worker::console_log!("{token:#?}");
            FormsResponse::ok("OK")
        },
    )
    .await
}
