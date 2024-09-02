pub mod db;

use worker::{Response, ResponseBuilder};

pub type WorkerHttpResponse = worker::Result<Response>;
pub type HttpResponse = Result<Response, Response>;

pub const CONTENT_TYPE: &str = "content-type";

pub struct FormsResponse;

impl FormsResponse {
    /// Returns error if any name is invalid (eg. contains space)
    pub fn headers(headers: &[(&str, &str)]) -> Result<worker::Headers, worker::Error> {
        let mut h = worker::Headers::new();

        for header in headers {
            h.set(header.0, header.1)
                .inspect_err(|err| worker::console_error!("Setting header: {err}"))?;
        }

        Ok(h)
    }

    /// Returns error if any name is invalid (eg. contains space)
    pub fn with_headers(headers: &[(&str, &str)]) -> Result<ResponseBuilder, Response> {
        let mut b = Response::builder();

        for header in headers {
            b = b
                .with_header(header.0, header.1)
                .inspect_err(|err| worker::console_error!("Setting header: {err}"))
                .map_err(|_| {
                    Response::builder()
                        .with_status(500)
                        .fixed(b"Internal Server Error".to_vec())
                })?;
        }

        Ok(b)
    }

    pub fn text(status: u16, body: impl Into<String>) -> Result<Response, Response> {
        let b =
            Self::with_headers(&[(CONTENT_TYPE, "text/plain; charset=utf-8")])?.with_status(status);

        Ok(b.fixed(body.into().into_bytes()))
    }

    pub fn json<T: serde::ser::Serialize>(status: u16, body: &T) -> Result<Response, Response> {
        let b = Self::with_headers(&[(CONTENT_TYPE, "application/json; charset=utf-8")])?
            .with_status(status);

        let body = serde_json::to_string(body).map_err(IntoResponse::into_response)?;

        Ok(b.fixed(body.into_bytes()))
    }

    pub fn ok(body: impl Into<String>) -> Result<Response, Response> {
        Self::text(200, body)
    }
}

pub trait IntoResponse {
    fn into_response(self) -> worker::Response;
}

impl IntoResponse for Result<worker::Response, worker::Response> {
    fn into_response(self) -> worker::Response {
        match self {
            Ok(a) => a,
            Err(b) => b,
        }
    }
}

impl IntoResponse for worker::Response {
    fn into_response(self) -> worker::Response {
        self
    }
}

impl IntoResponse for worker::Error {
    fn into_response(self) -> worker::Response {
        worker::console_error!("{self}");

        // TODO: Manage all error messages individually

        worker::ResponseBuilder::new()
            .with_status(500)
            .with_header(CONTENT_TYPE, "application/json; charset=utf-8")
            .unwrap()
            .fixed(format!(r#"{{"errors":[{:?}],"success":false}}"#, self.to_string()).into_bytes())
    }
}

impl IntoResponse for serde_json::Error {
    fn into_response(self) -> worker::Response {
        worker::console_error!("{self}");

        // TODO: Manage errors with more detail

        ResponseBuilder::new()
            .with_status(400)
            .with_header(CONTENT_TYPE, "application/json; charset=utf-8")
            .unwrap()
            .fixed(format!(r#"{{"errors":[{:?}],"success":false}}"#, self.to_string()).into_bytes())
    }
}

impl IntoResponse for worker::Result<worker::Response> {
    fn into_response(self) -> worker::Response {
        match self {
            Ok(o) => o,
            Err(err) => err.into_response(),
        }
    }
}

pub fn string_into_response(status: u16) -> impl Fn(String) -> worker::Response {
    move |data| {
        worker::Response::builder()
            .with_status(status)
            .fixed(data.into_bytes())
    }
}

const BEARER_PREFIX: &str = "Bearer ";
const BEARER_PREFIX_LEN: usize = BEARER_PREFIX.len();
pub const AUTH_TOKEN_LEN: usize = 36;

/// Get a valid authorization token.
pub fn get_auth(req: &mut worker::Request) -> Result<Option<String>, worker::Response> {
    req.headers()
        .get("authorization")
        .map(|auth| {
            auth.filter(|auth| {
                auth.starts_with(BEARER_PREFIX) && auth.len() == BEARER_PREFIX_LEN + AUTH_TOKEN_LEN
            })
            // SAFETY: Length is verified in the filter above
            .map(|auth| auth[BEARER_PREFIX_LEN..].to_owned())
        })
        .map_err(IntoResponse::into_response)
}

pub async fn get_body<T: serde::de::DeserializeOwned>(
    req: &mut worker::Request,
) -> Result<T, worker::Response> {
    let text = req.text().await.map_err(IntoResponse::into_response)?;

    serde_json::from_str::<T>(&text).map_err(IntoResponse::into_response)
}
