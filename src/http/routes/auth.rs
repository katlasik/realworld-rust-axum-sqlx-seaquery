use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::post;
use tracing::info;
use crate::http::dto::login::LoginRequest;
use crate::http::dto::register::RegisterRequest;
use crate::http::dto::user::{User, UserResponse};

pub(crate) fn auth_routes() -> Router {
  Router::new()
    .route("/users/login", post(login))
    .route("/users", post(register))
}

async fn login(
  Json(payload): Json<LoginRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
  info!("Login attempt for email: {}", payload.user.email);

  // TODO: Validate credentials against database
  let user = User {
    email: payload.user.email,
    token: "mock.jwt.token".to_string(),
    username: "mockuser".into(),
    bio: Some("I am a mock user".to_string()),
    image: None,
  };

  Ok(Json(UserResponse { user }))
}

async fn register(
  Json(payload): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
  info!("Registration attempt for email: {}", payload.user.email);

  // TODO: Validate and create user in database
  let user = User {
    email: payload.user.email,
    token: "mock.jwt.token".to_string(),
    username: payload.user.username,
    bio: None,
    image: None,
  };

  Ok(Json(UserResponse { user }))
}
