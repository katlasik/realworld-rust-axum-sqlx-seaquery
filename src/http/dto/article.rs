use serde::{Deserialize, Serialize};
use crate::http::dto::profile::Profile;

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleResponse {
    pub article: Article,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticlesResponse {
    pub articles: Vec<ArticleListItem>,
    #[serde(rename = "articlesCount")]
    pub articles_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    #[serde(rename = "tagList")]
    pub tag_list: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub favorited: bool,
    #[serde(rename = "favoritesCount")]
    pub favorites_count: i64,
    pub author: Profile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleListItem {
    pub slug: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "tagList")]
    pub tag_list: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub favorited: bool,
    #[serde(rename = "favoritesCount")]
    pub favorites_count: i64,
    pub author: Profile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateArticleRequest {
    pub article: CreateArticle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateArticle {
    pub title: String,
    pub description: String,
    pub body: String,
    #[serde(rename = "tagList")]
    pub tag_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateArticleRequest {
    pub article: UpdateArticle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateArticle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

#[derive(Deserialize)]
pub struct ArticleListQuery {
  tag: Option<String>,
  author: Option<String>,
  favorited: Option<String>,
  limit: Option<i64>,
  offset: Option<i64>,
}

