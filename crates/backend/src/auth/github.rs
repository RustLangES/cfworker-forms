use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use forms_shared::{FormsResponse, IntoResponse};
use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    EndpointNotSet, EndpointSet, RedirectUrl, Scope, StandardTokenResponse, TokenResponse,
    TokenUrl,
};
use serde::Deserialize;
use worker::{Method, Request, RequestInit};

use crate::auth::shared::vec_u8_to_uint8;
use crate::RouterContext;

use super::shared::Oauth2Token;

pub const ENV_CLIENT_ID: &str = "GH_CLIENT_ID";
pub const ENV_CLIENT_SECRET: &str = "GH_CLIENT_SECRET";

pub const URL_AUTH: &str = "https://github.com/login/oauth/authorize";
pub const URL_TOKEN: &str = "https://github.com/login/oauth/access_token";
pub const PATH_CALLBACK: &str = "/api/login/github/callback";

pub type GithubClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

pub type GithubToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

pub fn client_id(ctx: &RouterContext) -> ClientId {
    ClientId::new(
        ctx.env
            .secret(ENV_CLIENT_ID)
            .unwrap_or_else(|_| panic!("Missing the {ENV_CLIENT_ID} environment variable."))
            .to_string(),
    )
}

pub fn client_secret(ctx: &RouterContext) -> ClientSecret {
    ClientSecret::new(
        ctx.env
            .secret(ENV_CLIENT_SECRET)
            .unwrap_or_else(|_| panic!("Missing the {ENV_CLIENT_SECRET} environment variable."))
            .to_string(),
    )
}

pub fn auth_url() -> AuthUrl {
    AuthUrl::new(URL_AUTH.to_string()).expect("Invalid authorization endpoint URL")
}

pub fn token_url() -> TokenUrl {
    TokenUrl::new(URL_TOKEN.to_string()).expect("Invalid token endpoint URL")
}

pub fn client(ctx: &RouterContext) -> GithubClient {
    let host = ctx
        .env
        .var("HOST")
        .expect("Missing the HOST environment variable.");

    BasicClient::new(client_id(ctx))
        .set_client_secret(client_secret(ctx))
        .set_auth_uri(auth_url())
        .set_token_uri(token_url())
        .set_redirect_uri(
            RedirectUrl::new(format!("{host}{PATH_CALLBACK}")).expect("Invalid redirect URL"),
        )
}

fn authorize_url(redirect_to: Option<String>) -> oauth2::CsrfToken {
    if let Some(redirect_to) = redirect_to {
        let rand = CsrfToken::new_random_len(3);
        let rand = rand.secret();

        let state = format!("url%{rand}%{redirect_to}");

        serde_json::from_value::<CsrfToken>(serde_json::json!(state)).unwrap()
    } else {
        CsrfToken::new_random()
    }
}

pub fn get_authorize(ctx: &RouterContext, redirect_to: Option<String>) -> worker::Url {
    let (authorize_url, _) = client(ctx)
        .authorize_url(|| authorize_url(redirect_to))
        .add_scope(Scope::new("user:email".to_string()))
        .url();

    authorize_url
}

pub async fn get_token(
    ctx: RouterContext,
    code: String,
) -> (RouterContext, Result<Oauth2Token, worker::Response>) {
    let http_client = |req: oauth2::HttpRequest| callback(&ctx, req);

    let client = client(&ctx);
    let token = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(&http_client)
        .await
        .map_err(|err| FormsResponse::text(500, err.to_string()).unwrap_or_else(|a| a));

    (ctx, token)
}

pub async fn callback(
    ctx: &RouterContext,
    req: oauth2::HttpRequest,
) -> Result<oauth2::HttpResponse, worker::Error> {
    worker::console_log!("[GITHUB] ClientId and ClientSecret obtain");
    let client_id = client_id(ctx);
    let client_secret = client_secret(ctx);

    worker::console_log!("[GITHUB] Getting token");
    let urlencoded_id: String = form_urlencoded::byte_serialize(client_id.as_bytes()).collect();
    let urlencoded_secret: String =
        form_urlencoded::byte_serialize(client_secret.secret().as_bytes()).collect();
    let token = BASE64_STANDARD.encode(format!("{}:{}", &urlencoded_id, urlencoded_secret));

    let body = vec_u8_to_uint8(req.into_body());

    let headers = FormsResponse::headers(&[
        ("accept", "application/json"),
        ("content-type", "application/x-www-form-urlencoded"),
        ("authorization", &format!("Basic {token}")),
    ])
    .unwrap();

    let req = Request::new_with_init(
        URL_TOKEN,
        RequestInit::new()
            .with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(body.into())),
    )
    .unwrap();

    worker::console_log!("[GITHUB] Getting access token");
    let res = worker::Fetch::Request(req).send().await?;

    worker::console_log!("[GITHUB] Parse response");
    let res = worker::response_from_wasm(res.into())?;

    Result::<oauth2::HttpResponse, worker::Error>::Ok(res_worker_to_oauth2(res).await)
}

async fn res_worker_to_oauth2(res: worker::HttpResponse) -> oauth2::HttpResponse {
    let (parts, body) = res.into_parts();

    let mut stream = wasm_streams::ReadableStream::from_raw(body.into_inner().unwrap());
    let mut stream = stream.get_reader();
    let mut out_buffer = vec![];

    while let Ok(Some(buffer)) = stream.read().await {
        let buffer: worker::js_sys::Uint8Array = buffer.into();
        let mut buffer = buffer.to_vec();
        out_buffer.append(&mut buffer);
    }

    oauth2::HttpResponse::from_parts(parts, out_buffer)
}

/// ```json
/// {
///   "id": "integer",
///   "avatar_url": "string",
///   "name": "string",
///   "email": "string"
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct GithubUser {
    pub id: u64,
    // pub avatar_url: String,
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

/// https://docs.github.com/en/rest/users/emails?apiVersion=2022-11-28#list-email-addresses-for-the-authenticated-user
pub async fn get_user(token: GithubToken) -> Result<GithubUser, worker::Response> {
    let token = token.access_token().secret();

    let headers = FormsResponse::headers(&[
        ("accept", "application/vnd.github+json"),
        ("x-github-api-version", "2022-11-28"),
        ("user-agent", "@RustLangES Forms"),
        ("authorization", &format!("Bearer {token}")),
    ])
    .map_err(IntoResponse::into_response)?;

    let req = Request::new_with_init(
        "https://api.github.com/user",
        RequestInit::new()
            .with_method(Method::Get)
            .with_headers(headers),
    )
    .map_err(IntoResponse::into_response)?;

    worker::console_log!("[GITHUB] Getting user data");
    let mut res = worker::Fetch::Request(req)
        .send()
        .await
        .map_err(IntoResponse::into_response)?;

    let res = res.text().await.map_err(IntoResponse::into_response)?;

    let user = serde_json::from_str::<GithubUser>(&res).map_err(IntoResponse::into_response)?;

    Ok(user)
}
