use worker::{event, Context, Env, Request, Response, Result, Router};

// This event is called on start worker
// So we use the `start` event to initialize our tracing subscriber when the worker starts.
#[event(start)]
fn start() {
    // Custom panic
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(|info: &std::panic::PanicInfo| {
        worker::console_error!("{info}")
    }));
}

//
// Docs: https://github.com/cloudflare/workers-rs#or-use-the-router
// Example: https://github.com/cloudflare/workers-rs/blob/main/examples/router/src/lib.rs
//
#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new().run(req, env).await
}
