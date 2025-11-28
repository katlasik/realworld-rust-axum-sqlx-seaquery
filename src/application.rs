use crate::app_config::load_config;
use crate::database::connect_db;
use crate::persistence::article_repository::ArticleRepository;
use crate::persistence::comment_repository::CommentRepository;
use crate::persistence::profile_repository::ProfileRepository;
use crate::persistence::tag_repository::TagRepository;
use crate::persistence::user_repository::UserRepository;
use crate::server::init_server;
use crate::tracing::init_tracing;
use crate::utils::hasher::Hasher;
use crate::utils::jwt::JwtHandler;
use crate::{domain, http};
use domain::article_service::ArticleService;
use domain::comment_service::CommentService;
use domain::profile_service::ProfileService;
use domain::tag_service::TagService;
use domain::user_service::UserService;
use http::AppState;
use tracing::info;

pub async fn start_app() {
    let config = load_config();
    init_tracing(&config.tracing);
    info!("Starting realworld server...");

    let app_state = create_app_state(&config).await;

    init_server(&config.http, app_state)
        .await
        .expect("Failed to initialize server");
}

pub async fn create_app_state(config: &crate::app_config::AppConfig) -> AppState {
    let db = connect_db(&config.database)
        .await
        .expect("Failed to connect to database");

    let jwt = JwtHandler::new(config.secrets.jwt.0.clone());
    let hasher = Hasher::new(config.secrets.pepper.0.clone());

    let user_repo = UserRepository::new(db.clone());
    let article_repo = ArticleRepository::new(db.clone());
    let tag_repo = TagRepository::new(db.clone());
    let comment_repo = CommentRepository::new(db.clone());
    let profile_repo = ProfileRepository::new(db.clone());

    let user_service = UserService::new(user_repo, hasher);
    let article_service = ArticleService::new(article_repo, tag_repo.clone());
    let comment_service = CommentService::new(comment_repo);
    let tag_service = TagService::new(tag_repo);
    let profile_service = ProfileService::new(profile_repo);

    AppState {
        user_service,
        article_service,
        comment_service,
        tag_service,
        profile_service,
        config: config.clone(),
        jwt,
    }
}
