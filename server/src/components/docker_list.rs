use dioxus::prelude::*;

pub fn report_list(cx: Scope) -> Element {
    cx.render(rsx!(
        article {
            header {
                nav {
                    ul { li{ strong{ "Docker" } } }
                    ul { li{ a { href: "./reports/new", "New" } } }
                }
            }
            table {
                thead {
                    tr {
                        th { "Category" }
                        th { "Status" }
                    }
                }
                tbody {
                    tr {
                        td { "Active:" },
                        td {"0"},
                    }
                    tr {
                        td { "Insalled:" },
                        td { "15" },
                    }
                }
            }
        }
    ))
}
