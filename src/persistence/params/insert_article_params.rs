use crate::model::values::article_body::ArticleBody;
use crate::model::values::article_description::ArticleDescription;
use crate::model::values::article_title::ArticleTitle;
use crate::model::values::slug::Slug;
use crate::model::values::user_id::UserId;

pub struct InsertArticleParams {
    pub slug: Slug,
    pub title: ArticleTitle,
    pub description: ArticleDescription,
    pub body: ArticleBody,
    pub author_id: UserId,
}
