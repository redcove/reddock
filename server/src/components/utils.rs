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

pub fn nav(cx: Scope) -> Element {
    cx.render(rsx!(
        header {
            h2 {
                "/ ",
                a { href: "#", "Home" },
                "/ ",
            }
        }
    ))
}

pub fn foot(cx: Scope) -> Element {
    cx.render(rsx!(
        footer {
            class: "container",
            small {
                "Support ",
                a { href: "https://github.com/redcove/reddock", "Red Cove"},
                " | ",
                a { href: "https://github.com/redcove/reddock", "Source"},
            }
        }
    ))
}
