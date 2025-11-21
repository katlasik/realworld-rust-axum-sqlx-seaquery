use crate::http::dto::register::RegisterRequest;
use crate::model::values::email::Email;
use crate::model::values::password::Password;
use crate::model::values::username::Username;

pub struct RegisterCommand {
    pub(crate) username: Username,
    pub(crate) email: Email,
    pub(crate) password: Password,
}

impl RegisterCommand {
    pub(crate) fn new(req: RegisterRequest) -> Self {
        RegisterCommand {
            username: req.user.username,
            email: req.user.email,
            password: req.user.password,
        }
    }
}
