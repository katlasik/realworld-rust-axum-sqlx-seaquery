use crate::model::values::article_body::ArticleBody;
use crate::model::values::article_description::ArticleDescription;
use crate::model::values::article_id::ArticleId;
use crate::model::values::article_title::ArticleTitle;
use crate::model::values::slug::Slug;

pub struct UpdateArticleParams {
    pub article_id: ArticleId,
    pub slug: Option<Slug>,
    pub title: Option<ArticleTitle>,
    pub description: Option<ArticleDescription>,
    pub body: Option<ArticleBody>,
}

impl UpdateArticleParams {
    pub fn as_list(&self) -> Vec<(String, String)> {
        let mut fields = Vec::new();

        if let Some(slug) = &self.slug {
            fields.push(("slug".to_string(), slug.value().to_string()));
        }
        if let Some(title) = &self.title {
            fields.push(("title".to_string(), title.value().to_string()));
        }
        if let Some(description) = &self.description {
            fields.push(("description".to_string(), description.value().to_string()));
        }
        if let Some(body) = &self.body {
            fields.push(("body".to_string(), body.value().to_string()));
        }

        fields
    }
}
