use crate::app_error::AppError;
use crate::domain::commands::add_comment_command::AddCommentCommand;
use crate::model::persistence::comment_view::CommentView;
use crate::model::values::article_id::ArticleId;
use crate::model::values::comment_id::CommentId;
use crate::model::values::user_id::UserId;
use crate::persistence::comment_repository::CommentRepository;
use anyhow::Result;

#[derive(Clone)]
pub struct CommentService {
    comment_repo: CommentRepository,
}

impl CommentService {
    pub fn new(comment_repo: CommentRepository) -> Self {
        CommentService { comment_repo }
    }

    pub async fn delete_comment(
        &self,
        comment_id: CommentId,
        user_id: UserId,
    ) -> Result<(), AppError> {
        if !self
            .comment_repo
            .is_comment_author(comment_id, user_id)
            .await?
        {
            return Err(AppError::Forbidden);
        }

        self.comment_repo.delete_comment(comment_id).await
    }

    pub async fn add_comment(
        &self,
        command: AddCommentCommand,
        user_id: UserId,
    ) -> Result<CommentView, AppError> {
        let params = command.to_insert_params();
        let comment = self.comment_repo.insert_comment(params).await?;

        let comment = self
            .comment_repo
            .get_comment(comment.id, Some(user_id))
            .await?;

        Ok(comment)
    }

    pub async fn get_comments(
        &self,
        article_id: ArticleId,
        user_id: Option<UserId>,
    ) -> Result<Vec<CommentView>, AppError> {
        self.comment_repo.get_comments(article_id, user_id).await
    }
}
