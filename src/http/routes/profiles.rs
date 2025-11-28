use crate::app_error::AppError;
use crate::http::AppState;
use crate::http::dto::profile::{Profile, ProfileResponse};
use crate::http::extractors::auth_token::AuthToken;
use crate::model::values::username::Username;
use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use tracing::info;

pub(crate) fn profile_routes() -> Router<AppState> {
    Router::new()
        .route("/profiles/{username}", get(get_profile))
        .route("/profiles/{username}/follow", post(follow_user))
        .route("/profiles/{username}/follow", delete(unfollow_user))
}

#[utoipa::path(
    get,
    path = "/api/profiles/{username}",
    tag = "Profiles",
    params(
        ("username" = Username, Path, description = "Username of the profile to retrieve")
    ),
    responses(
        (status = 200, description = "Profile retrieved successfully", body = ProfileResponse),
        (status = 404, description = "Profile not found", body = crate::http::dto::error::ErrorResponse)
    )
)]
pub(crate) async fn get_profile(
    State(state): State<AppState>,
    auth: Option<AuthToken>,
    Path(username): Path<Username>,
) -> Result<Json<ProfileResponse>, AppError> {
    let maybe_user_id = auth.as_ref().map(|u| u.user_id);

    info!(user_id = ?maybe_user_id, username = %username, "Get profile for username: {}", username);

    let user = state
        .user_service
        .get_user_by_username(username.clone())
        .await?
        .ok_or_else(|| AppError::NotFound)?;

    let following = if let Some(user_id) = maybe_user_id {
        state.profile_service.is_following(user_id, user.id).await?
    } else {
        false
    };

    let profile = Profile::from_user(user, following);

    Ok(Json(ProfileResponse { profile }))
}

#[utoipa::path(
    post,
    path = "/api/profiles/{username}/follow",
    tag = "Profiles",
    params(
        ("username" = Username, Path, description = "Username of the profile to follow")
    ),
    responses(
        (status = 200, description = "User followed successfully", body = ProfileResponse),
        (status = 401, description = "Unauthorized - token missing or invalid", body = crate::http::dto::error::ErrorResponse),
        (status = 404, description = "Profile not found", body = crate::http::dto::error::ErrorResponse)
    )
)]
pub(crate) async fn follow_user(
    State(state): State<AppState>,
    auth: AuthToken,
    Path(username): Path<Username>,
) -> Result<Json<ProfileResponse>, AppError> {
    info!(user_id = %{auth.user_id}, username = %username, "Follow user: {}", username);

    let user = state
        .user_service
        .get_user_by_username(username)
        .await?
        .ok_or_else(|| AppError::NotFound)?;

    state
        .profile_service
        .follow_user(auth.user_id, user.id)
        .await?;

    let profile = Profile::from_user(user, true);

    Ok(Json(ProfileResponse { profile }))
}

#[utoipa::path(
    delete,
    path = "/api/profiles/{username}/follow",
    tag = "Profiles",
    params(
        ("username" = Username, Path, description = "Username of the profile to unfollow")
    ),
    responses(
        (status = 200, description = "User unfollowed successfully", body = ProfileResponse),
        (status = 401, description = "Unauthorized - token missing or invalid", body = crate::http::dto::error::ErrorResponse),
        (status = 404, description = "Profile not found", body = crate::http::dto::error::ErrorResponse)
    )
)]
pub(crate) async fn unfollow_user(
    State(state): State<AppState>,
    auth: AuthToken,
    Path(username): Path<Username>,
) -> Result<Json<ProfileResponse>, AppError> {
    info!(user_id = %{auth.user_id}, username = %username, "Unfollow user: {}", username);

    let user = state
        .user_service
        .get_user_by_username(username)
        .await?
        .ok_or_else(|| AppError::NotFound)?;

    state
        .profile_service
        .unfollow_user(auth.user_id, user.id)
        .await?;

    let profile = Profile::from_user(user, false);

    Ok(Json(ProfileResponse { profile }))
}
