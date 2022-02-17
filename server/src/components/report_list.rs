use dioxus::prelude::*;

pub fn report_list(cx: Scope) -> Element {
    cx.render(rsx!(
        h2 { "Reports" }
        table {
            class: "table",
            thead {
                tr {
                    th { "Name" }
                    th { "Updated" }
                }
            }
            tbody {
                tr {
                    td { a { href: "./reportid", "report 1"} },
                    td {"2022-02-22"},
                }
            }
        }
    ))
}
