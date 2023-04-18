use std::sync::Arc;

use axum::{http::StatusCode, extract::{Path, State}, Json};
use tokio::sync::watch;
use tracing::debug;
use uuid::Uuid;
use crate::{order_store::OrderStore};

use super::{response::Order, request::AddItem};

const USER_ID: Uuid = Uuid::from_u128(0x9cb4cf49_5c3d_4647_83b0_8f3515da7be1);
type DataState = Arc<dyn OrderStore + Sync + Send>;

//#[axum_macros::debug_handler]
pub async fn create(State(state): State<DataState>) -> (StatusCode, Json<Option<Order>>) {
  debug!("Create Order");
  if let Ok(order) = state.create_order(USER_ID).await {
    (StatusCode::OK, Json(Some(Order::from(order))))
  } else {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
  }
}

pub async fn list(State(state): State<DataState>) -> (StatusCode, Json<Option<Vec<Order>>>) {
  debug!("List Orders");
  if let Ok(orders) = state.list_orders(USER_ID).await {
    let orders: Vec<Order> = orders
    .into_iter()
    .map(|x| Order::from(x))
    .collect();
    (StatusCode::OK, Json(Some(orders)))
  } else {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
  }
}

pub async fn get(Path(id): Path<Uuid>, State(state): State<DataState>) -> (StatusCode, Json<Option<Order>>) {
  debug!("get order id: {id}");
  if let Ok(order) = state.get_order(id).await {
    (StatusCode::OK, Json(Some(Order::from(order))))
  } else {
    (StatusCode::NOT_FOUND, Json(None))
  }
}

pub async fn add_item(Path(id): Path<Uuid>, State(state): State<DataState>, Json(request): Json<AddItem>) -> StatusCode {
  debug!("Item added to the order id {id}");
  if let Ok(order) = state.get_order(id).await {
    match state.add_item(order.id, request.product_id, request.quantity).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    }
  } else {
    StatusCode::NOT_FOUND
  }
}

pub async fn delete_item(Path((id, index)): Path<(Uuid, usize)>, State(state): State<DataState>) -> StatusCode {
  debug!("delete item {index} from order id {id}");
  if let Ok(order) = state.get_order(id).await {
    match state.delete_item(order.id, index).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    }
  } else {
    StatusCode::NOT_FOUND
  }
}