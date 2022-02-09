use axum::{
    extract::{Extension},
    http::StatusCode,
    Json,
};
use bollard::Docker;
use bollard::models::SystemInfo;

pub async fn info(Extension(docker): Extension<Docker>) -> Result<Json<SystemInfo>, StatusCode> {

    let info = docker.info()
        .await
        .map_err(|e| {
            tracing::error!("Failed to get docker info: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(info))
}
