use axum::{
  extract::{FromRequestParts},
  http::{request::Parts, StatusCode},
};
use crate::http::AppState;
use crate::model::values::user_id::UserId;
use uuid::Uuid;

pub struct AuthToken{
  pub(crate) user_id: UserId,
  pub(crate) raw_token: String
}

impl FromRequestParts<AppState> for AuthToken
{
  type Rejection = (StatusCode, &'static str);

  async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {

    let jwt = &state.jwt;

    let auth = parts
      .headers
      .get(axum::http::header::AUTHORIZATION)
      .and_then(|h| h.to_str().ok())
      .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header"))?;

    let token = auth
      .strip_prefix("Bearer ")
      .ok_or((StatusCode::UNAUTHORIZED, "Invalid Bearer token"))?;


    let parsed_token = jwt.verify_token(token)
      .map_err(
        |_| (StatusCode::UNAUTHORIZED, "Invalid or expired token")
      )?;


    let uuid: Uuid = parsed_token.sub.parse()
      .map_err(|_| (StatusCode::UNAUTHORIZED, "Couldn't extract user id from token"))?;
    let user_id = UserId::from(uuid);

    Ok(AuthToken {
      user_id,
      raw_token: token.to_string()
    })
  }

}
