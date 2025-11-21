use crate::app_error::AppError;
use crate::database::Database;
use crate::model::persistence::comment::Comment;
use crate::model::persistence::comment_view::CommentView;
use crate::model::values::article_id::ArticleId;
use crate::model::values::comment_id::CommentId;
use crate::model::values::user_id::UserId;
use crate::persistence::params::insert_comment_params::InsertCommentParams;
use anyhow::Result;
use sqlx::{Postgres, QueryBuilder, Row};

#[derive(Clone)]
pub struct CommentRepository {
    database: Database,
}

fn comment_view_cte<'a>(user_id: Option<UserId>) -> QueryBuilder<'a, Postgres> {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r"
        WITH comment_view AS (
            SELECT
                c.id,
                c.body,
                c.created_at,
                c.updated_at,
                u.username as author_username,
                u.bio as author_bio,
                u.image as author_image
        "
    );

    if let Some(user_id) = user_id {
        query_builder.push(",
                EXISTS(
                    SELECT 1 FROM user_follows uf
                    WHERE uf.follower_id = ");
        query_builder.push_bind(user_id);
        query_builder.push("
                      AND uf.followee_id = u.id
                ) as following ");
    } else {
        query_builder.push(", FALSE as following ");
    }

    query_builder.push(
        r"
            FROM comments c
                INNER JOIN users u ON c.author_id = u.id
        )
        "
    );

    query_builder
}

impl CommentRepository {
    pub fn new(database: Database) -> Self {
        CommentRepository { database }
    }

    pub async fn insert_comment(&self, params: InsertCommentParams) -> Result<Comment, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO comments (body, article_id, author_id)
            VALUES ($1, $2, $3)
            RETURNING id, body, article_id, author_id, created_at, updated_at
            "#,
        )
        .bind(&params.body)
        .bind(params.article_id)
        .bind(params.author_id)
        .fetch_one(self.database.pool())
        .await?;

        Ok(Comment::from_row(row))
    }

    pub async fn delete_comment(&self, comment_id: CommentId) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM comments
            WHERE id = $1
            "#,
        )
        .bind(comment_id)
        .execute(self.database.pool())
        .await?;

        Ok(())
    }

    pub async fn is_comment_author(
        &self,
        comment_id: CommentId,
        user_id: UserId,
    ) -> Result<bool, AppError> {
        let row = sqlx::query(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM comments
                WHERE id = $1 AND author_id = $2
            ) as is_author
            "#,
        )
        .bind(comment_id)
        .bind(user_id)
        .fetch_one(self.database.pool())
        .await?;

        Ok(row.get("is_author"))
    }

    pub async fn get_comments(
        &self,
        article_id: ArticleId,
        user_id: Option<UserId>,
    ) -> Result<Vec<CommentView>, AppError> {
        let mut query = comment_view_cte(user_id);

        query.push(format!(
            "SELECT {} FROM comment_view WHERE id IN (SELECT id FROM comments WHERE article_id = ",
            CommentView::column_names("comment_view").join(", ")
        ));
        query.push_bind(article_id);
        query.push(") ORDER BY created_at DESC");

        let mut sql_query = sqlx::query(query.sql());

        if user_id.is_some() {
            sql_query = sql_query.bind(user_id);
        }

        sql_query = sql_query.bind(article_id);

        let rows = sql_query.fetch_all(self.database.pool()).await?;

        Ok(rows.into_iter().map(CommentView::from_row).collect())
    }

    pub async fn get_comment(
        &self,
        comment_id: CommentId,
        user_id: Option<UserId>,
    ) -> Result<CommentView, AppError> {
        let mut query = comment_view_cte(user_id);

        query.push(format!(
            "SELECT {} FROM comment_view WHERE id = ",
            CommentView::column_names("comment_view").join(", ")
        ));
        query.push_bind(comment_id);

        let mut sql_query = sqlx::query(query.sql());

        if user_id.is_some() {
            sql_query = sql_query.bind(user_id);
        }

        sql_query = sql_query.bind(comment_id);

        let row = sql_query.fetch_one(self.database.pool()).await?;

        Ok(CommentView::from_row(row))
    }
}
