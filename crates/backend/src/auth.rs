use oauth2::AuthorizationCode;
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet,
    RedirectUrl, Scope, TokenUrl,
};

use forms_shared::{FormsResponse, IntoResponse, WorkerHttpResponse};

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
    error_wrapper(req, ctx, |mut req, ctx, db| async move {
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

        let http_client = reqwest::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        let token = client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(&http_client)
            .await
            .map_err(|err| FormsResponse::text(500, err.to_string()).map_or_else(|a| a, |b| b))?;

        FormsResponse::ok("OK")
    })
    .await
}
