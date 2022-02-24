use dioxus::prelude::*;

pub fn head(cx: Scope) -> Element {
    cx.render(rsx!(
        head {
            meta { name: "viewport", content: "width=device-width, initial-scale=1" }
            meta { charset: "utf-8" }
            title { "Red Dock" }
            style { [include_str!("../../static/pico.min.css")] }
        }
    ))
}
