use std::convert::Infallible;
use std::{future, iter};

use oauth2::AuthorizationCode;
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet,
    RedirectUrl, Scope, TokenUrl,
};

use forms_shared::{FormsResponse, IntoResponse, WorkerHttpResponse};
use worker::{AbortSignal, ByteStream, Cf, Request, RequestInit};

use crate::shared::error_wrapper;
use crate::RouterContext;

fn get_github_client(
    ctx: &RouterContext,
) -> BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet> {
    let github_client_id = ClientId::new(
        ctx.env
            .var("GITHUB_CLIENT_ID")
            .expect("Missing the GITHUB_CLIENT_ID environment variable.")
            .to_string(),
    );
    let github_client_secret = ClientSecret::new(
        ctx.env
            .var("GITHUB_CLIENT_SECRET")
            .expect("Missing the GITHUB_CLIENT_SECRET environment variable.")
            .to_string(),
    );
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
        .expect("Invalid token endpoint URL");

    BasicClient::new(github_client_id)
        .set_client_secret(github_client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:8787/api/login/github/callback".to_string())
                .expect("Invalid redirect URL"),
        )
}

pub async fn github(req: worker::Request, ctx: RouterContext) -> WorkerHttpResponse {
    error_wrapper(req, ctx, |req, ctx, db| async move {
        let client = get_github_client(&ctx);

        let (authorize_url, _) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("user:email".to_string()))
            .url();

        worker::console_log!("GITHUB LOGIN:\n{authorize_url}\n");

        worker::Response::redirect(authorize_url).map_err(IntoResponse::into_response)
    })
    .await
}

pub async fn github_callback(req: worker::Request, ctx: RouterContext) -> WorkerHttpResponse {
    error_wrapper(req, ctx, |req, ctx, db| async move {
        let client = get_github_client(&ctx);

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

        let http_client = |req: oauth2::http::Request<Vec<u8>>| async {
            // let uri = "https://github.com/login/oauth/access_token";

            worker::console_log!("{req:#?}");
            let req = worker::request_to_wasm(req_oauth2_to_worker(req))?;
            // let body = req.into_body();
            // let body = worker::js_sys::Uint8Array::new_with_length(body.len() as u32);

            // let req = Request::new_with_init(
            //     uri,
            //     &RequestInit::new()
            //         .with_method(worker::Method::Post)
            //         .with_body(Some(body.into())),
            // );

            let res = worker::Fetch::Request(req.into()).send().await?;
            worker::console_log!("{res:#?}");

            let res = worker::response_from_wasm(res.into())?;

            Result::<oauth2::HttpResponse, worker::Error>::Ok(res_worker_to_oauth2(res).await)
        };

        let token = client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(&http_client)
            .await
            .map_err(|err| FormsResponse::text(500, err.to_string()).map_or_else(|a| a, |b| b))?;

        println!("{token:#?}");

        FormsResponse::ok("OK")
    })
    .await
}

struct BodyVec8(Vec<u8>);

impl futures_core::Stream for BodyVec8 {
    type Item = Result<Vec<u8>, Infallible>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        std::task::Poll::Ready(Some(Ok(self.0.clone())))
    }
}

fn req_oauth2_to_worker(req: oauth2::HttpRequest) -> worker::HttpRequest {
    let (parts, body) = req.into_parts();

    let body = worker::Body::from_stream(BodyVec8(body)).unwrap();

    worker::HttpRequest::from_parts(parts, body)
}

async fn res_worker_to_oauth2(res: worker::HttpResponse) -> oauth2::HttpResponse {
    let (parts, body) = res.into_parts();

    let mut stream = wasm_streams::ReadableStream::from_raw(body.into_inner().unwrap());
    let mut stream = stream.get_reader();
    let a = vec![];

    while let Ok(Some(b)) = stream.read().await {
        worker::console_log!("{b:#?}");
    }

    oauth2::HttpResponse::from_parts(parts, a)
}
