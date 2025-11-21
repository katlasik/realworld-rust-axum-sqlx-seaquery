use crate::application::start_app;

mod app_config;
mod app_error;
mod application;
mod database;
mod domain;
mod http;
mod model;
mod persistence;
mod server;
mod tracing;
mod utils;

#[tokio::main]
async fn main() {
    start_app().await
}
