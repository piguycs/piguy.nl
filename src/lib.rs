mod component;
mod page;

use worker::*;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    Router::new()
        .get("/", page::portfolio)
        .get("/calc/subnet", page::calc::subnet)
        .post_async("/calc/subnet", page::calc::subnet_post)
        .run(req, env)
        .await
}
