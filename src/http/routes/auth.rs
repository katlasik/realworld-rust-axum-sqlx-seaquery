use crate::app_error::AppError;
use crate::domain::commands::login_command::LoginCommand;
use crate::domain::commands::register_command::RegisterCommand;
use crate::http::AppState;
use crate::http::dto::login::LoginRequest;
use crate::http::dto::register::RegisterRequest;
use crate::http::dto::user::{UserData, UserResponse};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use tracing::info;

pub(crate) fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/users/login", post(login))
        .route("/users", post(register))
}

#[utoipa::path(
    post,
    path = "/api/users/login",
    tag = "Authentication",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = UserResponse),
        (status = 401, description = "Invalid credentials", body = crate::http::dto::error::ErrorResponse),
        (status = 422, description = "Validation error", body = crate::http::dto::error::ErrorResponse)
    )
)]
pub(crate) async fn login(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<UserResponse>, AppError> {
    info!("Login attempt for email: {}", payload.user.email);

    let command = LoginCommand::from_request(payload);

    let user = app_state.user_service.login_user(command).await?;

    let token = app_state.jwt.generate_token(user.id)?;

    let user = UserData::new(user, token);

    Ok(Json(UserResponse { user }))
}

#[utoipa::path(
    post,
    path = "/api/users",
    tag = "Authentication",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 422, description = "Validation error or user already exists", body = crate::http::dto::error::ErrorResponse)
    )
)]
pub(crate) async fn register(
    State(app_state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    info!(
        "Registration attempt for email: {}, username: {}, hash of password: {}",
        payload.user.email, payload.user.username, payload.user.password
    );

    let command = RegisterCommand::from_request(payload);

    let user = app_state.user_service.register_user(command).await?;

    let token = app_state.jwt.generate_token(user.id)?;

    let user = UserData {
        email: user.email,
        token,
        username: user.username,
        bio: user.bio,
        image: user.image,
    };

    Ok((StatusCode::CREATED, Json(UserResponse { user })))
}
