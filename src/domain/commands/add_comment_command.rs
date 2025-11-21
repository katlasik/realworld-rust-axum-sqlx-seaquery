use crate::http::dto::comment::CreateCommentRequest;
use crate::model::values::article_id::ArticleId;
use crate::model::values::comment_body::CommentBody;
use crate::model::values::user_id::UserId;
use crate::persistence::params::insert_comment_params::InsertCommentParams;

#[derive(Debug, Clone)]
pub struct AddCommentCommand {
    pub body: CommentBody,
    pub article_id: ArticleId,
    pub author_id: UserId,
}

impl AddCommentCommand {
    pub fn from_request(
        request: CreateCommentRequest,
        article_id: ArticleId,
        author_id: UserId,
    ) -> Self {
        AddCommentCommand {
            body: request.comment.body,
            article_id,
            author_id,
        }
    }

    pub fn to_insert_params(&self) -> InsertCommentParams {
        InsertCommentParams {
            body: self.body.clone(),
            article_id: self.article_id,
            author_id: self.author_id,
        }
    }
}
