use crate::http::dto::login::LoginRequest;
use crate::model::values::email::Email;
use crate::model::values::password::Password;

#[derive(Debug, Clone)]
pub struct LoginCommand {
    pub email: Email,
    pub password: Password,
}

impl LoginCommand {
    pub fn from_request(dto: LoginRequest) -> Self {
        LoginCommand {
            email: dto.user.email,
            password: dto.user.password,
        }
    }
}
