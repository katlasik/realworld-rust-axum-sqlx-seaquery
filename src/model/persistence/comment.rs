use crate::model::values::article_id::ArticleId;
use crate::model::values::comment_body::CommentBody;
use crate::model::values::comment_id::CommentId;
use crate::model::values::user_id::UserId;
use chrono::{DateTime, Utc};
use sqlx::Row;
use sqlx::postgres::PgRow;

pub struct Comment {
    pub id: CommentId,
    pub body: CommentBody,
    pub article_id: ArticleId,
    pub author_id: UserId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Comment {
    pub fn from_row(row: PgRow) -> Self {
        Self {
            id: row.get("id"),
            body: row.get("body"),
            article_id: row.get("article_id"),
            author_id: row.get("author_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
