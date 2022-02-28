use axum::response::Html;
use dioxus::prelude::*;
use crate::components::{utils, docker_list, report_list};

pub async fn render() -> Html<String> {
    fn homepage(cx: Scope) -> Element {
        cx.render(rsx!(
            utils::head()
            body {
                main {
                    class: "container",
                    utils::nav()
                    div {
                        class: "grid overviews",
                        id: "Overviews",
                        report_list::list() 
                        docker_list::list()
                    }
                }
                utils::foot()
            }
        ))
    }
    let mut app = VirtualDom::new(homepage);
    let _ = app.rebuild();
    Html(dioxus::ssr::render_vdom(&app))
}
