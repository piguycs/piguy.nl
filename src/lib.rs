mod component;
mod root_handler;

use root_handler::portfolio;
use worker::*;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    Router::new().get("/", portfolio).run(req, env).await
}
