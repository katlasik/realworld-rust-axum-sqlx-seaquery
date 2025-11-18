use axum::{Json, Router};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{get, post, delete};
use tracing::info;
use crate::http::dto::comment::{Comment, CommentResponse, CommentsResponse, CreateCommentRequest};
use crate::http::dto::profile::Profile;

pub(crate) fn comment_routes() -> Router {
  Router::new()
    .route("/articles/{slug}/comments", post(add_comment))
    .route("/articles/{slug}/comments", get(get_comments))
    .route("/articles/{slug}/comments/{id}", delete(delete_comment))
}

async fn add_comment(
  Path(slug): Path<String>,
  Json(payload): Json<CreateCommentRequest>,
) -> Result<Json<CommentResponse>, StatusCode> {
  info!("Add comment to article: {}", slug);

  // TODO: Save comment to database
  let comment = Comment {
    id: 1,
    created_at: "2024-01-01T00:00:00.000Z".to_string(),
    updated_at: "2024-01-01T00:00:00.000Z".to_string(),
    body: payload.comment.body,
    author: Profile {
      username: "currentuser".into(),
      bio: None,
      image: None,
      following: false,
    },
  };

  Ok(Json(CommentResponse { comment }))
}

async fn get_comments(
  Path(slug): Path<String>,
) -> Result<Json<CommentsResponse>, StatusCode> {
  info!("Get comments for article: {}", slug);

  // TODO: Fetch comments from database
  let comments = vec![];

  Ok(Json(CommentsResponse { comments }))
}

async fn delete_comment(
  Path((slug, id)): Path<(String, i64)>,
) -> Result<StatusCode, StatusCode> {
  info!("Delete comment {} from article: {}", id, slug);

  // TODO: Delete comment from database
  Ok(StatusCode::NO_CONTENT)
}
