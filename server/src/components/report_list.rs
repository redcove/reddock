use dioxus::prelude::*;

pub fn list(cx: Scope) -> Element {
    cx.render(rsx!(
        article {
            header {
                nav {
                    ul { li{ strong{ "Reports" } } }
                    ul { li{ a { href: "./reports/new", "New" } } }
                }
            }
            table {
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
        }
    ))
}
