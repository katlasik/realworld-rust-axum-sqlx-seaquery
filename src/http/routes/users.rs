use crate::app_error::AppError;
use crate::domain::commands::update_user_command::UpdateUserCommand;
use crate::http::AppState;
use crate::http::dto::user::{UpdateUserRequest, UserData, UserResponse};
use crate::http::extractors::auth_token::AuthToken;
use axum::extract::State;
use axum::routing::{get, put};
use axum::{Json, Router};
use tracing::info;

pub(crate) fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/user", get(get_current_user))
        .route("/user", put(update_user))
}

#[utoipa::path(
    get,
    path = "/api/user",
    tag = "User",
    responses(
        (status = 200, description = "Current user retrieved successfully", body = UserResponse),
        (status = 401, description = "Unauthorized - token missing or invalid", body = crate::http::dto::error::ErrorResponse)
    )
)]
pub(crate) async fn get_current_user(
    State(app_state): State<AppState>,
    auth_user: AuthToken,
) -> Result<Json<UserResponse>, AppError> {
    info!(user_id = %{auth_user.user_id}, "Get current user with id: {}", auth_user.user_id);

    let user = app_state
        .user_service
        .get_user_by_id(auth_user.user_id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let user = UserData {
        email: user.email,
        token: auth_user.raw_token,
        username: user.username,
        bio: user.bio,
        image: user.image,
    };

    Ok(Json(UserResponse { user }))
}

#[utoipa::path(
    put,
    path = "/api/user",
    tag = "User",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = UserResponse),
        (status = 401, description = "Unauthorized - token missing or invalid", body = crate::http::dto::error::ErrorResponse),
        (status = 422, description = "Validation error", body = crate::http::dto::error::ErrorResponse)
    )
)]
pub(crate) async fn update_user(
    State(app_state): State<AppState>,
    auth_user: AuthToken,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    info!(user_id = %{auth_user.user_id}, payload = ?payload, "Update user with id: {}", auth_user.user_id);

    let command = UpdateUserCommand::from_request(payload, auth_user.user_id);

    let user = app_state.user_service.update_user(command).await?;

    let user_date = UserData::new(user, auth_user.raw_token);

    Ok(Json(UserResponse { user: user_date }))
}
