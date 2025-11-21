use crate::http::AppState;
use crate::model::values::user_id::UserId;
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use uuid::Uuid;

pub struct AuthToken {
    pub(crate) user_id: UserId,
    pub(crate) raw_token: String,
}

impl FromRequestParts<AppState> for Option<AuthToken> {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jwt = &state.jwt;

        let maybe_raw_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok());

        if let Some(raw_header) = maybe_raw_header {
            let token = raw_header
                .strip_prefix("Bearer ")
                .ok_or((StatusCode::UNAUTHORIZED, "Invalid Bearer token"))?;

            let parsed_token = jwt
                .verify_token(token)
                .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token"))?;

            let uuid: Uuid = parsed_token.sub.parse().map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Couldn't extract user id from token",
                )
            })?;
            let user_id = UserId::from(uuid);

            Ok(Some(AuthToken {
                user_id,
                raw_token: token.to_string(),
            }))
        } else {
            Ok(None)
        }
    }
}

impl FromRequestParts<AppState> for AuthToken {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let maybe_token = Option::<AuthToken>::from_request_parts(parts, state).await?;
        maybe_token.ok_or((StatusCode::UNAUTHORIZED, "Authorization token required"))
    }
}
