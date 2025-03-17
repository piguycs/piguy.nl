use maud::*;

pub fn into_response(markup: Markup) -> worker::Result<worker::Response> {
    worker::Response::from_html(markup.into_string())
}

pub fn base(title: &str, styles: &str, body: Markup, htmx: bool) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en";
        (head(title, styles, htmx))
        body { (body) }
    }
}

pub fn blog_listing() -> Markup {
    html! {
        div {
            h3 { "Nothing here yet" }
        }
    }
}

fn head(title: &str, styles: &str, htmx: bool) -> Markup {
    html! {
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            @if htmx { script src="https://unpkg.com/htmx.org@2.0.4" {} }
            style { (styles) }
            title { (title) }
        }
    }
}
