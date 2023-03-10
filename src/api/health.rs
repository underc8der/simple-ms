use axum::http::StatusCode;
use tracing::info;

#[tracing::instrument]
pub async fn get() -> StatusCode {
    info!("Health status requested");
    StatusCode::OK
}