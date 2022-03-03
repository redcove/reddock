use dioxus::prelude::*;

// Our report type as stored in the DB
#[derive(PartialEq, Clone)]
pub struct Report {
    id: i64,
    name: String,
    updated: i64,
}

#[derive(Props, PartialEq, Clone)]
pub struct ReportProps {
    pub reports: Vec<Report>,
}

pub fn ReportList(cx: Scope<ReportProps>) -> Element {
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
                    cx.props.reports.iter().map(|report| rsx!(
                        tr {
                            key: "{report.id}",
                            td {a { href: "/reports/{report.id}", "{report.name}"} },
                            td { "{report.updated}" },
                        }
                    ))
                }
            }
        }
    ))
}
