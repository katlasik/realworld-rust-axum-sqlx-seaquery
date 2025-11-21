use crate::http::dto::user::UpdateUserRequest;
use crate::model::values::bio::Bio;
use crate::model::values::email::Email;
use crate::model::values::image::Image;
use crate::model::values::password::Password;
use crate::model::values::user_id::UserId;
use crate::model::values::username::Username;

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
  pub(crate) fn new(req: UpdateUserRequest, user_id: UserId) -> Self {
      UpdateUserCommand {
          user_id,
          email: req.user.email,
          username: req.user.username,
          password: req.user.password,
          bio: req.user.bio,
          image: req.user.image,
      }
  }
}
