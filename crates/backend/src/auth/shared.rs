use std::future::Future;

use oauth2::{basic::BasicTokenType, EmptyExtraTokenFields, StandardTokenResponse};

use forms_shared::{FormsResponse, WorkerHttpResponse};

use crate::shared::error_wrapper;
use crate::RouterContext;

pub type Oauth2Token = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

pub async fn auth_callback<R, RouteResponse>(
    req: worker::Request,
    ctx: RouterContext,
    get_token: impl Fn(RouterContext, String) -> R + Clone,
    route: impl Fn(
            worker::Request,
            RouterContext,
            worker::D1Database,
            StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
        ) -> RouteResponse
        + Clone,
) -> WorkerHttpResponse
where
    RouteResponse: Future<Output = Result<worker::Response, worker::Response>>,
    R: Future<Output = (RouterContext, Result<Oauth2Token, worker::Response>)>,
{
    error_wrapper(req, ctx, |req, ctx, db| {
        let get_token = get_token.clone();
        let route = route.clone();

        async move {
            let Some(code) = req
                .url()
                .unwrap()
                .query_pairs()
                .find(|n| n.0 == "code")
                .map(|n| n.1.to_string())
            else {
                return FormsResponse::json(
                    400,
                    &serde_json::json!({
                        "errors": ["Malformed query"],
                        "success": false
                    }),
                );
            };

            let (ctx, token) = get_token(ctx, code).await;

            route(req, ctx, db, token?).await
        }
    })
    .await
}

pub fn vec_u8_to_uint8(value: Vec<u8>) -> worker::js_sys::Uint8Array {
    let array = worker::js_sys::Uint8Array::new_with_length(value.len() as u32);
    array.copy_from(&value);
    array
}
