use crate::app_error::AppError;
use crate::database::Database;
use crate::model::values::user_id::UserId;
use anyhow::Result;
use sqlx::Row;

#[derive(Clone)]
pub struct ProfileRepository {
    database: Database,
}

impl ProfileRepository {
    pub fn new(database: Database) -> Self {
        ProfileRepository { database }
    }

    pub async fn follow_user(
        &self,
        follower_id: UserId,
        followee_id: UserId,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO user_follows (follower_id, followee_id)
            VALUES ($1, $2)
            ON CONFLICT (follower_id, followee_id) DO NOTHING
            "#,
        )
        .bind(follower_id)
        .bind(followee_id)
        .execute(self.database.pool())
        .await?;

        Ok(())
    }

    pub async fn unfollow_user(
        &self,
        follower_id: UserId,
        followee_id: UserId,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM user_follows
            WHERE follower_id = $1 AND followee_id = $2
            "#,
        )
        .bind(follower_id)
        .bind(followee_id)
        .execute(self.database.pool())
        .await?;

        Ok(())
    }

    pub async fn is_following(
        &self,
        follower_id: UserId,
        followee_id: UserId,
    ) -> Result<bool, AppError> {
        let row = sqlx::query(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM user_follows
                WHERE follower_id = $1 AND followee_id = $2
            ) as is_following
            "#,
        )
        .bind(follower_id)
        .bind(followee_id)
        .fetch_one(self.database.pool())
        .await?;

        Ok(row.get("is_following"))
    }
}
