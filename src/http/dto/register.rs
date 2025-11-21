use crate::model::values::email::Email;
use crate::model::values::password::Password;
use crate::model::values::username::Username;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub user: RegisterUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    pub username: Username,
    pub email: Email,
    pub password: Password,
}
