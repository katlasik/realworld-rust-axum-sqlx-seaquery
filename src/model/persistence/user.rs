use crate::model::values::bio::Bio;
use crate::model::values::email::Email;
use crate::model::values::image::Image;
use crate::model::values::password_hash::PasswordHash;
use crate::model::values::user_id::UserId;
use crate::model::values::username::Username;
use sqlx::Row;
use sqlx::postgres::PgRow;

pub struct User {
    pub id: UserId,
    pub email: Email,
    pub password_hash: PasswordHash,
    pub username: Username,
    pub bio: Option<Bio>,
    pub image: Option<Image>,
}

impl User {
    pub fn from_row(row: PgRow) -> Self {
        Self {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            bio: row.get("bio"),
            image: row.get("image"),
        }
    }
}
