use crate::app_config::{LogFormatting, TracingConfig};
use tracing_subscriber::layer::{Layered, SubscriberExt};
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

pub fn init_tracing(tracing_config: &TracingConfig) {
    let subscriber: Layered<EnvFilter, Registry, Registry> =
        tracing_subscriber::registry().with(EnvFilter::new(
            tracing_config
                .level
                .clone()
                .unwrap_or("realworld=debug,tower_http=debug,axum=trace".into()),
        ));

    match tracing_config.formatting {
        LogFormatting::Pretty => subscriber
            .with(tracing_subscriber::fmt::layer().pretty())
            .init(),
        LogFormatting::Json => subscriber
            .with(tracing_subscriber::fmt::layer().json())
            .init(),
    };
}
