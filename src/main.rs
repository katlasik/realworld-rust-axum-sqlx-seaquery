use axum::Router;
use tracing::log::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::app_config::load_config;
use crate::http::router;

mod app_config;
mod http;


#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "realworld=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting realworld server...");

    let config = load_config();

    info!("Server running on {}", config.http.url());

    let listener = tokio::net::TcpListener::bind(config.http.url())
        .await
        .unwrap();

    let router = Router::new()
      .nest("/api", router());

    axum::serve(listener, router)
        .await
        .unwrap();
}
