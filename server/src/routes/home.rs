use axum::{
    response::Html,
    extract::{Extension},
    http::StatusCode,
};
use sqlx::sqlite::SqlitePool;
use dioxus::prelude::*;
use crate::components::{
    utils, 
    docker_list, 
    report_list::{Report, ReportList, ReportProps}
};

pub async fn render(Extension(db): Extension<SqlitePool>) -> Result<Html<String>, StatusCode> {
    let recs = sqlx::query!(
        r#"
        SELECT id, name, scope, description, updated, created
        FROM reports
        ORDER BY id
        "#
    )
    .fetch_all(&db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to get list of reports from db: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let reports: Vec<Report> = recs
        .into_iter()
        .map(|rec| Report {
            id: rec.id,
            name: rec.name,
            updated: rec.updated,
        })
        .collect();

    fn homepage(cx: Scope<ReportProps>) -> Element {
        cx.render(rsx!(
            utils::head()
            body {
                main {
                    class: "container",
                    utils::nav()
                    div {
                        class: "grid overviews",
                        id: "Overviews",
                        ReportList {reports: props.reports} 
                        docker_list::list()
                    }
                }
                utils::foot()
            }
        ))
    }
    let mut app = VirtualDom::new_with_props(homepage, ReportProps{reports});
    let _ = app.rebuild();
    Ok(Html(dioxus::ssr::render_vdom(&app)))
}
