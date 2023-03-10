use axum::{
    error_handling::HandleErrorLayer,
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::{post, delete, get}, 
    BoxError, Router, Server,
};

use dotenv::dotenv;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use std::{env};
use tracing::{error, info};

use std::time::Duration;
use tower::timeout::TimeoutLayer;
use api::{health, orders};

mod order_store;
mod in_mem_order_store;
mod api;

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

    let app: Router = Router::new()
        .route("/", get(|| async { "Super Microservice" }))
        .route("/hello", get(hello))
        .route("/health", get(health::get))
        .route("/orders", get(orders::list).post(orders::create))
        .route("/orders/:id", get(orders::get))
        .route("/orders/:id/item", post(orders::add_item))
        .route("/orders/:id/item/:index", delete(orders::delete_item))
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



async fn hello() -> &'static str {
    // tokio::time::sleep(Duration::from_secs(6)).await;
    "Supermicroserv in hello fn"
}

#[tracing::instrument]
async fn fallback_handler(uri: Uri) -> impl IntoResponse {
    error!("No route for {}", uri);
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}
