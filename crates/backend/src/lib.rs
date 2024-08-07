mod admins;
mod auth;
mod form;
mod question;
mod session;
mod shared;

use worker::{event, Context, Env, Request, Response, Result, Router};

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

    router = router
        // General Login
        .get_async("/api/login/github", auth::github)
        .get_async("/api/login/github/callback", auth::github_callback);

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
        // Admin Session
        .get_async("/api/session", session::get_admin)
        // General Session
        .get_async("/api/form/:form_id/session", session::get)
        // Unique Session
        .delete_async("/api/form/:form_id/session", session::delete);

    router.run(req, env).await
}
