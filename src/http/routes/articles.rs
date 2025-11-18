use axum::{Json, Router};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::routing::{get, post, put, delete};
use tracing::info;
use crate::http::dto::article::{Article, ArticleResponse, ArticlesResponse, CreateArticleRequest, UpdateArticleRequest, ArticleListQuery};
use crate::http::dto::profile::Profile;

pub(crate) fn article_routes() -> Router {
  Router::new()
    .route("/articles", get(list_articles))
    .route("/articles/feed", get(feed_articles))
    .route("/articles/{slug}", get(get_article))
    .route("/articles", post(create_article))
    .route("/articles/{slug}", put(update_article))
    .route("/articles/{slug}", delete(delete_article))
    .route("/articles/{slug}/favorite", post(favorite_article))
    .route("/articles/{slug}/favorite", delete(unfavorite_article))
}


async fn list_articles(
  Query(params): Query<ArticleListQuery>,
) -> Result<Json<ArticlesResponse>, StatusCode> {
  info!("List articles with filters");

  // TODO: Fetch articles from database with filters
  let articles = vec![];

  Ok(Json(ArticlesResponse {
    articles,
    articles_count: 0,
  }))
}

async fn feed_articles(
  Query(params): Query<ArticleListQuery>,
) -> Result<Json<ArticlesResponse>, StatusCode> {
  info!("Get article feed");

  // TODO: Fetch articles from followed users
  let articles = vec![];

  Ok(Json(ArticlesResponse {
    articles,
    articles_count: 0,
  }))
}

async fn get_article(
  Path(slug): Path<String>,
) -> Result<Json<ArticleResponse>, StatusCode> {
  info!("Get article: {}", slug);

  // TODO: Fetch article from database
  let article = Article {
    slug: slug.clone(),
    title: "Mock Article".to_string(),
    description: "Mock description".to_string(),
    body: "Mock body content".to_string(),
    tag_list: vec!["mock".to_string()],
    created_at: "2024-01-01T00:00:00.000Z".to_string(),
    updated_at: "2024-01-01T00:00:00.000Z".to_string(),
    favorited: false,
    favorites_count: 0,
    author: Profile {
      username: "mockauthor".into(),
      bio: None,
      image: None,
      following: false,
    },
  };

  Ok(Json(ArticleResponse { article }))
}

async fn create_article(
  Json(payload): Json<CreateArticleRequest>,
) -> Result<Json<ArticleResponse>, StatusCode> {
  info!("Create article: {}", payload.article.title);

  // TODO: Create slug from title and save to database
  let slug = payload.article.title
    .to_lowercase()
    .replace(" ", "-");

  let article = Article {
    slug: slug.clone(),
    title: payload.article.title,
    description: payload.article.description,
    body: payload.article.body,
    tag_list: payload.article.tag_list,
    created_at: "2024-01-01T00:00:00.000Z".to_string(),
    updated_at: "2024-01-01T00:00:00.000Z".to_string(),
    favorited: false,
    favorites_count: 0,
    author: Profile {
      username: "currentuser".into(),
      bio: None,
      image: None,
      following: false,
    },
  };

  Ok(Json(ArticleResponse { article }))
}

async fn update_article(
  Path(slug): Path<String>,
  Json(payload): Json<UpdateArticleRequest>,
) -> Result<Json<ArticleResponse>, StatusCode> {
  info!("Update article: {}", slug);

  // TODO: Update article in database
  let new_slug = payload.article.title
    .as_ref()
    .map(|t| t.to_lowercase().replace(" ", "-"))
    .unwrap_or(slug.clone());

  let article = Article {
    slug: new_slug,
    title: payload.article.title.unwrap_or("Updated Article".to_string()),
    description: payload.article.description.unwrap_or("Updated description".to_string()),
    body: payload.article.body.unwrap_or("Updated body".to_string()),
    tag_list: vec![],
    created_at: "2024-01-01T00:00:00.000Z".to_string(),
    updated_at: "2024-01-01T00:00:00.000Z".to_string(),
    favorited: false,
    favorites_count: 0,
    author: Profile {
      username: "currentuser".into(),
      bio: None,
      image: None,
      following: false,
    },
  };

  Ok(Json(ArticleResponse { article }))
}

async fn delete_article(
  Path(slug): Path<String>,
) -> Result<StatusCode, StatusCode> {
  info!("Delete article: {}", slug);

  // TODO: Delete article from database
  Ok(StatusCode::NO_CONTENT)
}

async fn favorite_article(
  Path(slug): Path<String>,
) -> Result<Json<ArticleResponse>, StatusCode> {
  info!("Favorite article: {}", slug);

  // TODO: Add favorite relationship in database
  let article = Article {
    slug: slug.clone(),
    title: "Mock Article".to_string(),
    description: "Mock description".to_string(),
    body: "Mock body content".to_string(),
    tag_list: vec![],
    created_at: "2024-01-01T00:00:00.000Z".to_string(),
    updated_at: "2024-01-01T00:00:00.000Z".to_string(),
    favorited: true,
    favorites_count: 1,
    author: Profile {
      username: "mockauthor".into(),
      bio: None,
      image: None,
      following: false,
    },
  };

  Ok(Json(ArticleResponse { article }))
}

async fn unfavorite_article(
  Path(slug): Path<String>,
) -> Result<Json<ArticleResponse>, StatusCode> {
  info!("Unfavorite article: {}", slug);

  // TODO: Remove favorite relationship from database
  let article = Article {
    slug: slug.clone(),
    title: "Mock Article".to_string(),
    description: "Mock description".to_string(),
    body: "Mock body content".to_string(),
    tag_list: vec![],
    created_at: "2024-01-01T00:00:00.000Z".to_string(),
    updated_at: "2024-01-01T00:00:00.000Z".to_string(),
    favorited: false,
    favorites_count: 0,
    author: Profile {
      username: "mockauthor".into(),
      bio: None,
      image: None,
      following: false,
    },
  };

  Ok(Json(ArticleResponse { article }))
}
