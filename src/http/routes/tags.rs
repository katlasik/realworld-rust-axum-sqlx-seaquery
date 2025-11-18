use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;
use tracing::info;
use crate::http::dto::tag::TagsResponse;

pub(crate) fn tag_routes() -> Router {
  Router::new()
    .route("/tags", get(get_tags))
}

async fn get_tags() -> impl IntoResponse {
  info!("Get tags");

  // TODO: Fetch tags from database
  let tags = vec![
    "mock".to_string(),
    "test".to_string(),
    "demo".to_string(),
  ];

  Json(TagsResponse { tags }).into_response()
}
