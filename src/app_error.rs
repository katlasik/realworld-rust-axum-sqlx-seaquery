use crate::http::dto::error::ErrorResponse;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Resource not found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Bad request: {0}")]
    BadData(String),
    #[error("Conflict: {0}")]
    DataConflict(String),
    #[error("Database error")]
    Db(#[from] sqlx::Error),
    #[error("Internal error: {0}")]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                Json::from(ErrorResponse::new("Not found".into())),
            )
                .into_response(),

            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json::from(ErrorResponse::new("Unauthorized".into())),
            )
                .into_response(),

            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                Json::from(ErrorResponse::new("Forbidden".into())),
            )
                .into_response(),

            AppError::BadData(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json::from(ErrorResponse::new(msg)),
            )
                .into_response(),

            AppError::Db(err) => {
                error!("Database error: {err:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json::from(ErrorResponse::new("Database error".into())),
                )
                    .into_response()
            }

            AppError::DataConflict(msg) => {
                (StatusCode::CONFLICT, Json::from(ErrorResponse::new(msg))).into_response()
            }

            AppError::Other(err) => {
                error!("Internal: {err:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json::from(ErrorResponse::new("Internal server error".into())),
                )
                    .into_response()
            }
        }
    }
}
