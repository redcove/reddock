use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;

// Our report type as stored in the DB
#[derive(Debug, Deserialize, Serialize)]
pub struct Report {
    id: i64,
    name: String,
    scope: String,
    description: Option<String>,
    created: i64,
    updated: i64,
}

// Request to create new report
#[derive(Debug, Deserialize)]
pub struct CreateReport {
    name: String,
    scope: String,
    description: Option<String>,
}

pub async fn create(
    Json(input): Json<CreateReport>,
    Extension(db): Extension<SqlitePool>,
) -> Result<Json<Report>, StatusCode> {
    // Get timestamp and make the report
    let timestamp = Utc::now().timestamp();
    let mut report = Report {
        id: 0,
        name: input.name,
        scope: input.scope,
        description: input.description,
        created: timestamp,
        updated: timestamp,
    };

    // Save it to the db
    let id = sqlx::query!(
        r#"
        INSERT INTO reports (name, scope, description, created, updated)
        VALUES(?1, ?2, ?3, ?4, ?5)
        "#,
        report.name,
        report.scope,
        report.description,
        timestamp,
        timestamp
    )
    .execute(&db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create report in db: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .last_insert_rowid();

    // If saved send the created report back
    report.id = id;
    Ok(Json(report))
}

// The query parameters for todos index
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

// Report list
#[derive(Debug, Deserialize, Serialize)]
pub struct ReportList {
    pub reports: Vec<Report>,
}

pub async fn list(Extension(db): Extension<SqlitePool>) -> Result<Json<Vec<Report>>, StatusCode> {
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
            scope: rec.scope,
            description: rec.description,
            updated: rec.updated,
            created: rec.created,
        })
        .collect();

    Ok(Json(reports))
}

pub async fn get(
    Path(id): Path<i64>,
    Extension(db): Extension<SqlitePool>,
) -> Result<Json<Report>, StatusCode> {
    // Fetch the report
    let rec = sqlx::query!(
        r#"
        SELECT id, name, scope, description, updated, created
        FROM reports
        WHERE id = ?1
        "#,
        id
    )
    .fetch_one(&db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to get report from db: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let report = Report {
        id: rec.id,
        name: rec.name,
        scope: rec.scope,
        description: rec.description,
        updated: rec.updated,
        created: rec.created,
    };

    // If saved send the updated report back
    Ok(Json(report))
}

#[derive(Debug, Deserialize)]
pub struct UpdateReport {
    name: Option<String>,
    scope: Option<String>,
    description: Option<String>,
}

pub async fn update(
    Path(id): Path<i64>,
    Json(input): Json<UpdateReport>,
    Extension(db): Extension<SqlitePool>,
) -> Result<Json<Report>, StatusCode> {
    // Fetch the old report
    let rec = sqlx::query!(
        r#"
        SELECT id, name, scope, description, updated, created
        FROM reports
        WHERE id = ?1
        "#,
        id
    )
    .fetch_one(&db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to get report while updating: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Update timestamp
    let timestamp = Utc::now().timestamp();
    // See if we need to update scope
    let scope = if let Some(new_scope) = input.scope {
        update_scope(rec.scope, new_scope)
    } else {
        rec.scope
    };

    let report = Report {
        id: rec.id,
        name: input.name.unwrap_or(rec.name),
        scope,
        description: if let Some(desc) = input.description {
            Some(desc)
        } else {
            rec.description
        },
        updated: timestamp,
        created: rec.created,
    };

    // Save it to the db
    sqlx::query!(
        r#"
        INSERT INTO reports (name, scope, description, created, updated)
        VALUES(?1, ?2, ?3, ?4, ?5)
        "#,
        report.name,
        report.scope,
        report.description,
        timestamp,
        timestamp
    )
    .execute(&db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update report in db: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // If saved send the updated report back
    Ok(Json(report))
}

// If new scope exists in old scope it removes it otherwise adds it to the scope
fn update_scope(old: String, new: String) -> String {
    let mut old_scope: Vec<&str> = old.split(',').collect();

    for new_scope in new.split(',') {
        let mut add = true;
        old_scope.retain(|old| {
            if old == &new_scope {
                add = false
            }
            add
        });
        if add {
            old_scope.push(new_scope)
        }
    }

    "".to_string()
}

pub async fn delete(
    Path(id): Path<i64>,
    Extension(db): Extension<SqlitePool>,
) -> Result<impl IntoResponse, StatusCode> {
    // Fetch the old report
    sqlx::query!(
        r#"
        DELETE FROM reports
        WHERE id = ?1
        "#,
        id
    )
    .execute(&db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to delete report from db: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(StatusCode::OK)
}
