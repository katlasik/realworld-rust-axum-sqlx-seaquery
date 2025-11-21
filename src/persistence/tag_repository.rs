use crate::app_error::AppError;
use crate::database::Database;
use crate::model::persistence::tag::Tag;
use crate::model::values::tag_name::TagName;
use crate::persistence::params::insert_tag_params::InsertTagParams;
use anyhow::Result;
use sqlx::Row;

#[derive(Clone)]
pub struct TagRepository {
    database: Database,
}

impl TagRepository {
    pub fn new(database: Database) -> Self {
        TagRepository { database }
    }

    pub async fn insert_tag(&self, params: InsertTagParams) -> Result<Tag, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO tags (name)
            VALUES ($1)
            RETURNING id, name, created_at
            "#,
        )
        .bind(&params.name)
        .fetch_one(self.database.pool())
        .await?;

        Ok(Tag::from_row(row))
    }

    pub async fn get_tag_by_name(&self, name: &TagName) -> Result<Option<Tag>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, name, created_at
            FROM tags
            WHERE name = $1
            "#,
        )
        .bind(name)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(row.map(Tag::from_row))
    }

    pub async fn get_all_tags(&self) -> Result<Vec<TagName>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT name
            FROM tags
            ORDER BY name
            "#,
        )
        .fetch_all(self.database.pool())
        .await?;

        Ok(rows.into_iter().map(|row| row.get("name")).collect())
    }

    pub async fn get_or_create_tag(&self, name: &TagName) -> Result<Tag, AppError> {
        if let Some(tag) = self.get_tag_by_name(name).await? {
            Ok(tag)
        } else {
            self.insert_tag(InsertTagParams { name: name.clone() })
                .await
        }
    }
}
