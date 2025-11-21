use axum::Router;
use axum::body::Body;
use rand::Rng;
use realworld::app_config::{AppConfig, DatabaseConfig, HttpConfig};
use realworld::application::create_app_state;
use realworld::database::connect_db;
use realworld::http::router;
use realworld::tracing::init_tracing;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tryphon::{Config, EnvOverrides};

pub async fn create_test_app() -> Router {
    let db = TestDatabase::new("realworld".to_string(), "password".to_string())
        .await
        .unwrap();

    let mut env_overrides = EnvOverrides::init();

    env_overrides.set("DATABASE_NAME", &db.name);

    let config = AppConfig::load().unwrap();

    let app_state = create_app_state(&config).await;

    router(app_state)
}

struct TestDatabase {
    name: String,
}

fn random_string() -> String {
    let mut rand = rand::thread_rng();
    (0..25).map(|_| rand.gen_range('a'..'z')).collect()
}

impl TestDatabase {
    async fn new(user: String, password: String) -> anyhow::Result<TestDatabase> {
        let name = format!("test_db_{}", random_string().to_lowercase());

        let db = PgPoolOptions::new()
            .max_connections(1)
            .connect(format!("postgresql://{}:{}@localhost:5432/postgres", user, password).as_str())
            .await?;

        let query = format!(r#" CREATE DATABASE  {}"#, name);

        sqlx::query(&query).execute(&db).await?;

        info!("Created test database: {}", name);

        Ok(TestDatabase { name })
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        let test_db_name = self.name.clone();

        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            handle.spawn(async move {
                let config = AppConfig::load().unwrap();
                let db = PgPoolOptions::new()
                    .max_connections(1)
                    .connect(&config.database.connection_url())
                    .await
                    .unwrap();

                let query = format!(r#" DROP DATABASE IF EXISTS {}"#, test_db_name);

                info!("Dropping test database: {}", test_db_name);

                sqlx::query(&query).execute(&db).await.unwrap()
            });
        }
    }
}
