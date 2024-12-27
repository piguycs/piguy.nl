use maud::html;
use worker::*;

use crate::component::{base, into_response};

pub fn subnet(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let body = html! {
        h1 { "WIP" }
    };

    into_response(base("Subnet Calculator", "", body, true))
}

pub fn subnet_post(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    todo!()
}
