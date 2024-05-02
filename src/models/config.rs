use serde::Deserialize;
use tokio::fs::read_to_string;
use tracing::{info, debug};

#[derive(Debug, Clone, Deserialize)]
pub struct Config{
    pub production: bool,
    pub title: String,
    pub url: String,
    pub port: u16,
    pub sleep_time: u64,
    pub per_page: i64,
    pub secret_key: String,
    pub admin_username: String,
    pub admin_password: String,
}

impl Config {
    pub async fn load() -> Self {
        info!("load");
        let content = read_to_string("config.yml")
            .await
            .expect("Can't read config file `config.yml`");
        debug!("Content: {content}");
        serde_yaml::from_str(&content)
            .expect("Can't process config file `config.yml`")
    }
}
