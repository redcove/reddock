use dioxus::prelude::*;

pub fn head(cx: Scope) -> Element {
    cx.render(rsx!(
        head {
            style { [include_str!("../../static/spectre.min.css")] }
        }
    ))
}
