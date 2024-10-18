use axum::{
    routing::{get, post},
    Router, Json, extract::Query,
    http::StatusCode,
    middleware,
};
use serde_json::json;
use std::net::SocketAddr;
use tracing::info;
use crate::handlers::*;
use crate::utils::log_requests;

mod handlers;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/create_event", post(create_event))
        .route("/update_event", post(update_event))
        .route("/delete_event", post(delete_event))
        .route("/events_for_day", get(events_for_day))
        .route("/events_for_week", get(events_for_week))
        .route("/events_for_month", get(events_for_month))
        .layer(middleware::from_fn(log_requests));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
