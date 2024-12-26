use maud::*;

pub fn into_response(markup: Markup) -> worker::Result<worker::Response> {
    worker::Response::from_html(markup.into_string())
}

pub fn base(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en";
        (head(title))
        body { (body) }
    }
}

pub fn blog_listing(title: &str) -> Markup {
    html! {
        div {
            h3 { (title) }
        }
    }
}

fn head(title: &str) -> Markup {
    html! {
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            style { (include_str!("../assets/style.css")) }
            title { (title) }
        }
    }
}
