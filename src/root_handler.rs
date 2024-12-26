use maud::html;
use worker::*;

use super::component::{base, blog_listing, into_response};

pub fn portfolio(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let github_url = ctx.var("GITHUB_URL").expect("github url not set");
    let codeberg_url = ctx.var("CODEBERG_URL").expect("codeberg url not set");
    let linkedin_url = ctx.var("LINKEDIN_URL").expect("linkedin url not set");

    let body = html! {
        h1 { "> Kunal Dandekar" }
        main {
            p { "Software Engineer" }
            div {
                a target="blank" href=(github_url) { button { "GitHub" } }
                a target="blank" href=(codeberg_url) { button { "Codeberg" } }
                a target="blank" href=(linkedin_url) { button { "Linkedin" } }
            }
            h2 { "Latest blogs" }
            div {
                (blog_listing("W.I.P."))
            }
        }

    };

    into_response(base("💻 Kunal Dandekar", body))
}
