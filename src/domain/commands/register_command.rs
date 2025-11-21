use crate::http::dto::register::RegisterRequest;
use crate::model::values::email::Email;
use crate::model::values::password::Password;
use crate::model::values::password_hash::PasswordHash;
use crate::model::values::username::Username;
use crate::persistence::params::insert_user_params::InsertUserParams;

pub struct RegisterCommand {
    pub(crate) username: Username,
    pub(crate) email: Email,
    pub(crate) password: Password,
}

impl RegisterCommand {
    pub(crate) fn from_request(dto: RegisterRequest) -> Self {
        RegisterCommand {
            username: dto.user.username,
            email: dto.user.email,
            password: dto.user.password,
        }
    }

    pub(crate) fn to_params(&self, password_hash: PasswordHash) -> InsertUserParams {
        InsertUserParams {
            email: self.email.clone(),
            username: self.username.clone(),
            password_hash,
        }
    }
}
