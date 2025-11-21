use crate::model::values::tag_id::TagId;
use crate::model::values::tag_name::TagName;
use chrono::{DateTime, Utc};
use sqlx::Row;
use sqlx::postgres::PgRow;

pub struct Tag {
    pub id: TagId,
    pub name: TagName,
    pub created_at: DateTime<Utc>,
}

impl Tag {
    pub fn from_row(row: PgRow) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            created_at: row.get("created_at"),
        }
    }
}
