use crate::http::dto::user::UpdateUserRequest;
use crate::model::values::bio::Bio;
use crate::model::values::email::Email;
use crate::model::values::image::Image;
use crate::model::values::password::Password;
use crate::model::values::password_hash::PasswordHash;
use crate::model::values::user_id::UserId;
use crate::model::values::username::Username;
use crate::persistence::params::update_user_params::UpdateUserParams;

#[derive(Debug, Clone)]
pub struct UpdateUserCommand {
    pub user_id: UserId,
    pub email: Option<Email>,
    pub username: Option<Username>,
    pub password: Option<Password>,
    pub bio: Option<Bio>,
    pub image: Option<Image>,
}

impl UpdateUserCommand {
    pub(crate) fn from_request(dto: UpdateUserRequest, user_id: UserId) -> Self {
        UpdateUserCommand {
            user_id,
            email: dto.user.email,
            username: dto.user.username,
            password: dto.user.password,
            bio: dto.user.bio,
            image: dto.user.image,
        }
    }

    pub(crate) fn to_params(&self, password_hash: Option<PasswordHash>) -> UpdateUserParams {
        UpdateUserParams {
            user_id: self.user_id,
            email: self.email.clone(),
            username: self.username.clone(),
            password_hash,
            bio: self.bio.clone(),
            image: self.image.clone(),
        }
    }
}
