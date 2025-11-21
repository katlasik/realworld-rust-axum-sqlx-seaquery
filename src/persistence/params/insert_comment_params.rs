use crate::model::values::article_id::ArticleId;
use crate::model::values::comment_body::CommentBody;
use crate::model::values::user_id::UserId;

pub struct InsertCommentParams {
    pub body: CommentBody,
    pub article_id: ArticleId,
    pub author_id: UserId,
}
