use axum::{http::StatusCode, extract::Path};
use tracing::debug;
use uuid::Uuid;

pub async fn create() -> StatusCode {
  debug!("Create Order");
  StatusCode::FORBIDDEN
}

pub async fn list() -> StatusCode {
  debug!("List Orders");
  StatusCode::FORBIDDEN
}

pub async fn get(Path(id): Path<Uuid>) -> StatusCode {
  debug!("get order id: {id}");
  StatusCode::FORBIDDEN
}

pub async fn add_item(Path(id): Path<Uuid>) -> StatusCode {
  debug!("Item added to the order id {id}");
  StatusCode::FORBIDDEN
}

pub async fn delete_item(Path((id, index)): Path<(Uuid, usize)>) -> StatusCode {
  debug!("delete item {index} from order id {id}");
  StatusCode::FORBIDDEN
}