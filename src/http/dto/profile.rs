use crate::model::persistence::user::User;
use crate::model::values::bio::Bio;
use crate::model::values::image::Image;
use crate::model::values::username::Username;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileResponse {
    pub profile: Profile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub username: Username,
    pub bio: Option<Bio>,
    pub image: Option<Image>,
    pub following: bool,
}

impl Profile {
    pub fn from_user(user: User, following: bool) -> Profile {
        Profile {
            username: user.username,
            bio: user.bio,
            image: user.image,
            following,
        }
    }
}
