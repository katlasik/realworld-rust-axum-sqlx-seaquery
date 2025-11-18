use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;

pub(crate) fn health_routes() -> Router {
  Router::new()
    .route("/health", get(health))
}

async fn health() -> impl IntoResponse {
  Json::from("OK").into_response()
}
