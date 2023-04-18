mod in_mem_order_store;
mod order_store;
mod api;

use axum::{
    error_handling::HandleErrorLayer,
    http::{StatusCode, Uri},
    response::{IntoResponse, ErrorResponse},
    routing::{post, delete, get}, 
    BoxError, Router, Server, Json, extract::Path,
};

use dotenv::dotenv;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use uuid::Uuid;
use std::{env, sync::Arc};
use tracing::{error, info};

use std::time::Duration;
use tower::timeout::TimeoutLayer;
use api::{health, orders};
use serde_json::{json, Value};

use crate::in_mem_order_store::InMemOrderStore;

/* 
    current video: https://youtu.be/mhpe2rFhedo?t=3270
 */
#[tokio::main]
async fn main() {
    
    tracing_subscriber::fmt::init();

    dotenv()
        .expect("Set your configuration in .env file!");
    let server_addr = env::var("SERVER")
        .expect("Define SERVER=host:port in your .env");
    let server_addr = server_addr
        .parse()
        .expect("Define SERVER=host:port in your .env");

    info!("Launching server: http://{server_addr}/");

    let repo = InMemOrderStore::new();
    let state = Arc::new(repo);

    let orders_route = Router::new()
        .route("/", get(orders::list).post(orders::create))
        .route("/:id", get(orders::get))
        .route("/:id/item", post(orders::add_item))
        .route("/:id/item/:index", delete(orders::delete_item))
        .with_state(state);

    let app: Router = Router::new()
        .route("/", get(|| async { "Super Microservice" }))
        .route("/hello", get(hello))
        .route("/hello/:msg", get(json_sample))
        .route("/health", get(health::get))
        .nest("/orders", orders_route)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(TimeoutLayer::new(Duration::from_secs(5))),
        )
        .fallback(fallback_handler);

    Server::bind(&server_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn json_sample(Path(msg): Path<String>) -> Result<Json<Value>, ErrorResponse> {
    let id = Uuid::new_v4();
    Ok(Json(json!({ "id": id, "message": msg})))
}

async fn hello() -> &'static str {
    // tokio::time::sleep(Duration::from_secs(6)).await;
    "Supermicroserv in hello fn"
}

#[tracing::instrument]
async fn fallback_handler(uri: Uri) -> impl IntoResponse {
    error!("No route for {}", uri);
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}
