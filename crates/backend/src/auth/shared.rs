use std::borrow::Borrow;
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
            Option<String>,
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
            worker::console_log!("Someone is trying to login");

            let query_pairs = req.url().unwrap();
            let query_pairs = query_pairs.query_pairs();

            let (code, state) =
                query_pairs.fold((None, None), |prev, (key, val)| match key.borrow() {
                    "code" => (Some(val.to_string()), prev.1),
                    "state" => (prev.0, Some(val.to_string())),
                    _ => prev,
                });

            let Some(code) = code else {
                return FormsResponse::json(
                    400,
                    &serde_json::json!({
                        "errors": ["Malformed query"],
                        "success": false
                    }),
                );
            };

            let redirect_to = state
                .filter(|state| state.starts_with("url%"))
                .and_then(|state| Some((state[6..].find("%")? + 7, state)))
                .map(|(start, state)| state[start..].to_string());

            let (ctx, token) = get_token(ctx, code).await;

            route(req, ctx, db, token?, redirect_to).await
        }
    })
    .await
}

pub fn vec_u8_to_uint8(value: Vec<u8>) -> worker::js_sys::Uint8Array {
    let array = worker::js_sys::Uint8Array::new_with_length(value.len() as u32);
    array.copy_from(&value);
    array
}
