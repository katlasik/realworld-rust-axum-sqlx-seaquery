use crate::http::AppState;
use crate::http::dto::tag::TagsResponse;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use tracing::info;
use crate::app_error::AppError;

pub(crate) fn tag_routes() -> Router<AppState> {
    Router::new().route("/tags", get(get_tags))
}

async fn get_tags(State(state): State<AppState>) -> Result<Json<TagsResponse>, AppError> {
    info!("Get tags");

    let tags = state
        .tag_service
        .get_all_tags()
        .await?;

    Ok(Json(TagsResponse { tags }))
}
