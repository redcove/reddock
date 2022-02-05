use axum::http::StatusCode;

// Simple test route that returns Status 200 if the server is up.
pub async fn handler() -> StatusCode {
    StatusCode::OK
}
