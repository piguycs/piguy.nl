mod component;
mod page;
mod subnetcalc;

use worker::*;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    Router::new()
        .get("/", page::portfolio)
        .get("/calc/subnet", page::subnetcalc)
        .run(req, env)
        .await
}
