pub(crate) mod dto;
pub(crate) mod extractors;
pub(crate) mod routes;

use axum::extract::Request;
use routes::*;

use crate::app_config::AppConfig;
use crate::domain::article_service::ArticleService;
use crate::domain::comment_service::CommentService;
use crate::domain::profile_service::ProfileService;
use crate::domain::tag_service::TagService;
use crate::domain::user_service::UserService;
use crate::openapi::ApiDoc;
use crate::utils::jwt::JwtHandler;
use axum::Router;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, MakeSpan, TraceLayer};
use tracing::Span;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
struct FilteringMakeSpan<'a> {
    inner: DefaultMakeSpan,
    exceptions: Vec<&'a str>,
}

impl<'a> FilteringMakeSpan<'a> {
    fn except_routes(exceptions: Vec<&'a str>) -> Self {
        Self {
            exceptions,
            inner: DefaultMakeSpan::new().level(tracing::Level::INFO),
        }
    }
}

impl<B> MakeSpan<B> for FilteringMakeSpan<'_> {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        if self.exceptions.contains(&request.uri().path()) {
            Span::none()
        } else {
            self.inner.make_span(request)
        }
    }
}

pub fn router(state: AppState) -> Router {
    let api_routes = Router::new()
        .merge(auth::auth_routes())
        .merge(users::user_routes())
        .merge(profiles::profile_routes())
        .merge(articles::article_routes())
        .merge(comments::comment_routes())
        .merge(tags::tag_routes())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(FilteringMakeSpan::except_routes(vec!["/api/health"]))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        .merge(health::health_routes());

    Router::new()
        .nest("/api", api_routes)
        .merge(SwaggerUi::new("/api/docs").url("/api/docs/openapi.json", ApiDoc::openapi()))
        .with_state(state)
}

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub user_service: UserService,
    pub article_service: ArticleService,
    pub comment_service: CommentService,
    pub tag_service: TagService,
    pub profile_service: ProfileService,
    pub jwt: JwtHandler,
}
