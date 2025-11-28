use crate::app_error::AppError;
use crate::domain::commands::create_article_command::CreateArticleCommand;
use crate::domain::commands::get_feed_query::GetFeedQuery;
use crate::domain::commands::list_articles_query::ListArticlesQuery;
use crate::domain::commands::update_article_command::UpdateArticleCommand;
use crate::http::AppState;
use crate::http::dto::article::{
    ArticleFeedListQuery, ArticleItem, ArticleListItem, ArticleListQuery, ArticleResponse,
    ArticlesResponse, CreateArticleRequest, UpdateArticleRequest,
};
use crate::http::extractors::auth_token::AuthToken;
use crate::model::values::slug::Slug;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use tracing::info;

pub(crate) fn article_routes() -> Router<AppState> {
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

#[utoipa::path(
    get,
    path = "/api/articles",
    tag = "Articles",
    params(ArticleListQuery),
    responses(
        (status = 200, description = "Articles retrieved successfully", body = ArticlesResponse)
    )
)]
pub(crate) async fn list_articles(
    State(state): State<AppState>,
    auth: Option<AuthToken>,
    Query(params): Query<ArticleListQuery>,
) -> Result<Json<ArticlesResponse>, AppError> {
    info!(params = ?params, "List articles with filters");

    let query = ListArticlesQuery::from_request(params);
    let user_id = auth.as_ref().map(|u| u.user_id);

    let articles = state
        .article_service
        .list_articles(query.clone(), user_id)
        .await?;
    let articles_count = state.article_service.count_articles(query, user_id).await?;

    let views: Vec<_> = articles
        .iter()
        .map(ArticleListItem::from_article_view)
        .collect();

    Ok(Json(ArticlesResponse {
        articles: views,
        articles_count,
    }))
}

#[utoipa::path(
    get,
    path = "/api/articles/feed",
    tag = "Articles",
    params(ArticleFeedListQuery),
    responses(
        (status = 200, description = "Feed articles retrieved successfully", body = ArticlesResponse),
        (status = 401, description = "Unauthorized - token missing or invalid", body = crate::http::dto::error::ErrorResponse)
    )
)]
pub(crate) async fn feed_articles(
    State(state): State<AppState>,
    auth: AuthToken,
    Query(params): Query<ArticleFeedListQuery>,
) -> Result<Json<ArticlesResponse>, AppError> {
    info!(params = ?params, "Get article feed");

    let query = GetFeedQuery::from_request(params, auth.user_id);

    let articles = state.article_service.get_feed(query).await?;

    let views: Vec<_> = articles
        .iter()
        .map(ArticleListItem::from_article_view)
        .collect();

    let articles_count = state
        .article_service
        .count_feed_articles(auth.user_id)
        .await?;
    Ok(Json(ArticlesResponse {
        articles: views,
        articles_count,
    }))
}

#[utoipa::path(
    get,
    path = "/api/articles/{slug}",
    tag = "Articles",
    params(
        ("slug" = Slug, Path, description = "Slug of the article to retrieve")
    ),
    responses(
        (status = 200, description = "Article retrieved successfully", body = ArticleResponse),
        (status = 404, description = "Article not found", body = crate::http::dto::error::ErrorResponse)
    )
)]
pub(crate) async fn get_article(
    State(state): State<AppState>,
    auth: Option<AuthToken>,
    Path(slug): Path<Slug>,
) -> Result<Json<ArticleResponse>, AppError> {
    info!(slug = %slug, "Get article: {}", slug);

    let article = state
        .article_service
        .get_article(&slug, auth.map(|u| u.user_id))
        .await?
        .ok_or_else(|| AppError::NotFound)?;

    let article = ArticleItem::from_article_view(&article);

    Ok(Json(ArticleResponse { article }))
}

#[utoipa::path(
    post,
    path = "/api/articles",
    tag = "Articles",
    request_body = CreateArticleRequest,
    responses(
        (status = 201, description = "Article created successfully", body = ArticleResponse),
        (status = 401, description = "Unauthorized - token missing or invalid", body = crate::http::dto::error::ErrorResponse),
        (status = 422, description = "Validation error", body = crate::http::dto::error::ErrorResponse)
    )
)]
pub(crate) async fn create_article(
    State(state): State<AppState>,
    auth: AuthToken,
    Json(payload): Json<CreateArticleRequest>,
) -> Result<(StatusCode, Json<ArticleResponse>), AppError> {
    info!(payload = ?payload, "Create article");

    let command = CreateArticleCommand::from_request(payload, auth.user_id);

    let article_view = state.article_service.create_article(command).await?;

    let article = ArticleItem::from_article_view(&article_view);

    Ok((StatusCode::CREATED, Json(ArticleResponse { article })))
}

#[utoipa::path(
    put,
    path = "/api/articles/{slug}",
    tag = "Articles",
    params(
        ("slug" = Slug, Path, description = "Slug of the article to update")
    ),
    request_body = UpdateArticleRequest,
    responses(
        (status = 200, description = "Article updated successfully", body = ArticleResponse),
        (status = 401, description = "Unauthorized - token missing or invalid", body = crate::http::dto::error::ErrorResponse),
        (status = 403, description = "Forbidden - not the article author", body = crate::http::dto::error::ErrorResponse),
        (status = 404, description = "Article not found", body = crate::http::dto::error::ErrorResponse),
        (status = 422, description = "Validation error", body = crate::http::dto::error::ErrorResponse)
    )
)]
pub(crate) async fn update_article(
    State(state): State<AppState>,
    auth: AuthToken,
    Path(slug): Path<Slug>,
    Json(payload): Json<UpdateArticleRequest>,
) -> Result<Json<ArticleResponse>, AppError> {
    info!(slug = %slug, payload = ?payload , "Update article: {}", slug);

    let command = UpdateArticleCommand::from_request(payload, slug);

    let updated_article = state
        .article_service
        .update_article(command, auth.user_id)
        .await?;

    let article = ArticleItem::from_article_view(&updated_article);

    Ok(Json(ArticleResponse { article }))
}

#[utoipa::path(
    delete,
    path = "/api/articles/{slug}",
    tag = "Articles",
    params(
        ("slug" = Slug, Path, description = "Slug of the article to delete")
    ),
    responses(
        (status = 204, description = "Article deleted successfully"),
        (status = 401, description = "Unauthorized - token missing or invalid", body = crate::http::dto::error::ErrorResponse),
        (status = 403, description = "Forbidden - not the article author", body = crate::http::dto::error::ErrorResponse),
        (status = 404, description = "Article not found", body = crate::http::dto::error::ErrorResponse)
    )
)]
pub(crate) async fn delete_article(
    State(state): State<AppState>,
    auth: AuthToken,
    Path(slug): Path<Slug>,
) -> Result<StatusCode, AppError> {
    info!(slug = %slug, "Delete article: {}", slug);

    state
        .article_service
        .delete_article(slug, auth.user_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/api/articles/{slug}/favorite",
    tag = "Articles",
    params(
        ("slug" = Slug, Path, description = "Slug of the article to favorite")
    ),
    responses(
        (status = 200, description = "Article favorited successfully", body = ArticleResponse),
        (status = 401, description = "Unauthorized - token missing or invalid", body = crate::http::dto::error::ErrorResponse),
        (status = 404, description = "Article not found", body = crate::http::dto::error::ErrorResponse)
    )
)]
pub(crate) async fn favorite_article(
    State(state): State<AppState>,
    auth: AuthToken,
    Path(slug): Path<Slug>,
) -> Result<Json<ArticleResponse>, AppError> {
    info!(slug = %slug, "Favorite article: {}", slug);

    state
        .article_service
        .favorite_article(auth.user_id, &slug)
        .await?;

    let article = state
        .article_service
        .get_article(&slug, Some(auth.user_id))
        .await?
        .ok_or_else(|| AppError::NotFound)?;

    let article = ArticleItem::from_article_view(&article);

    Ok(Json(ArticleResponse { article }))
}

#[utoipa::path(
    delete,
    path = "/api/articles/{slug}/favorite",
    tag = "Articles",
    params(
        ("slug" = Slug, Path, description = "Slug of the article to unfavorite")
    ),
    responses(
        (status = 200, description = "Article unfavorited successfully", body = ArticleResponse),
        (status = 401, description = "Unauthorized - token missing or invalid", body = crate::http::dto::error::ErrorResponse),
        (status = 404, description = "Article not found", body = crate::http::dto::error::ErrorResponse)
    )
)]
pub(crate) async fn unfavorite_article(
    State(state): State<AppState>,
    auth: AuthToken,
    Path(slug): Path<Slug>,
) -> Result<Json<ArticleResponse>, AppError> {
    info!(slug = %slug, "Unfavorite article: {}", slug);

    state
        .article_service
        .unfavorite_article(auth.user_id, &slug)
        .await?;

    let article = state
        .article_service
        .get_article(&slug, Some(auth.user_id))
        .await?
        .ok_or_else(|| AppError::NotFound)?;

    let article = ArticleItem::from_article_view(&article);

    Ok(Json(ArticleResponse { article }))
}
