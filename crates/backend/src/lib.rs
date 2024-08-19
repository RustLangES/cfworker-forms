mod admins;
mod answer;
mod auth;
mod form;
mod question;
mod session;
mod shared;

use worker::{event, Context, Env, Method, Request, Response, Result, Router};

pub type RouterContext = worker::RouteContext<()>;

#[event(start)]
fn start() {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(|info: &std::panic::PanicInfo| {
        worker::console_error!("{info}")
    }));
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let mut router = Router::new();

    // Prevent errors from preflight call
    // - https://developer.mozilla.org/en-US/docs/Glossary/Preflight_request
    // - https://developers.cloudflare.com/workers/examples/cors-header-proxy/
    router = router.options("/*path", |req, _| {
        let headers = req.headers();
        if headers.has("Origin")?
            && headers.has("Access-Control-Request-Method")?
            && headers.has("Access-Control-Request-Headers")?
        {
            // Handle CORS preflight requests.
            Ok(worker::Response::builder()
                .with_header("Access-Control-Allow-Origin", "*")?
                .with_header("Access-Control-Allow-Methods", "GET,HEAD,POST,OPTIONS")?
                .with_header("Access-Control-Max-Age", "86400")?
                .with_header(
                    "Access-Control-Allow-Headers",
                    &headers.get("Access-Control-Request-Headers")?.unwrap(),
                )?
                .empty())
        } else {
            Ok(worker::Response::builder()
                .with_header("Allow", "GET, HEAD, POST, OPTIONS")?
                .empty())
        }
    });

    router = router
        // General Login
        .get_async("/api/login/github", auth::github)
        .get_async("/api/login/github/callback", auth::github_callback);

    router = router
        // Admin Session
        .get_async("/api/session", session::get_admin)
        // General Session
        .get_async("/api/form/:form_id/session", session::get)
        // Unique Session
        .delete_async("/api/form/:form_id/session", session::delete);

    router = router
        // General Form
        .get_async("/api/form", form::get_all)
        .post_async("/api/form", form::post)
        // Unique Form
        .get_async("/api/form/:id", form::get)
        .patch_async("/api/form/:id", form::patch)
        .delete_async("/api/form/:id", form::delete);

    router = router
        // General Question
        .get_async("/api/form/:form_id/question", question::get_all)
        .post_async("/api/form/:form_id/question", question::post)
        // Unique Question
        .patch_async("/api/form/:form_id/question/:id", question::patch)
        .delete_async("/api/form/:form_id/question/:id", question::delete);

    router = router
        .get_async("/api/form/:form_id/answer", answer::get_all)
        // Unique Answer
        .get_async("/api/form/:form_id/question/:id/answer", answer::get)
        .post_async("/api/form/:form_id/question/:id/answer", answer::post);

    let method = req.method();
    let origin = &req.url()?.origin().ascii_serialization();

    let mut res = router.run(req, env).await?;

    if method != Method::Options {
        // CORS :\
        res.headers_mut()
            .set("Access-Control-Allow-Origin", "*")?;
        res.headers_mut().set("Vary", "Origin")?;
    }

    Ok(res)
}
