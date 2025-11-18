use tryphon::{Config, ErrorPrintMode};

#[derive(Debug, Config)]
pub struct HttpConfig {
    #[env("HTTP_HOST")] #[default("0.0.0.0")]
    pub(crate) host: String,
    #[env("HTTP_PORT")] #[default(8080)]
    pub(crate) port: u16,
}

impl HttpConfig {
    pub(crate) fn url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Config)]
pub struct AppConfig {
    #[config]
    pub(crate) http: HttpConfig
}

pub fn load_config() -> AppConfig {
  match AppConfig::load() {
    Ok(cfg) => cfg,
    Err(e) => {
      eprintln!("Couldn't load configuration from env variables:\n{}", e.pretty_print(ErrorPrintMode::Table));
      panic!("Configuration loading failed");
    }
  }
}