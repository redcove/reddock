use axum::{http::StatusCode, response::Html};
use dioxus::prelude::*;

// Simple test route that returns Status 200 if the server is up.
pub async fn status() -> StatusCode {
    StatusCode::OK
}


pub async fn ssr() -> Html<String> {
    Html(dioxus::ssr::render_lazy(rsx! {
        div {
            h1 { "hello world!" }
            p { "goodbye world!" }
            (0..10).map(|f| rsx!{
                p { "test: {f}" }
            })
        }
    }))
}
