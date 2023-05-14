use serde::{Serialize, Deserialize};
use serde_yaml::Error;

const DEFAULT_SLEEP_TIME: u64 = 1;
const DEFAULT_PER_PAGE: i64 = 10;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration{
    #[serde(default = "get_default_dev")]
    dev: bool,
    log_level: String,
    db_url: String,
    port: u16,
    #[serde(default = "get_default_sleep_time")]
    sleep_time: u64,
    url: String,
    #[serde(default = "get_default_per_page")]
    per_page: i64,
    cookies: String,
    jwt_secret: String,
    jwt_expires_in: String,
    jwt_maxage: i32,
}

fn get_default_dev() -> bool{
    return false;
}

fn get_default_sleep_time() -> u64{
    return DEFAULT_SLEEP_TIME;
}

fn get_default_per_page() -> i64{
    return DEFAULT_PER_PAGE;
}

impl Configuration {
    pub fn new(content: &str) -> Result<Configuration, Error>{
        serde_yaml::from_str(content)
    }
    pub fn get_log_level(&self) -> &str{
        &self.log_level
    }
    pub fn get_db_url(&self) -> &str{
        &self.db_url
    }
    pub fn get_dev(&self) -> bool{
        self.dev
    }
    pub fn get_sleep_time(&self) -> u64{
        self.sleep_time
    }
    pub fn get_page(&self) -> i64{
        self.per_page
    }
    pub fn get_port(&self) -> u16{
        self.port
    }
    pub fn get_url(&self) -> &str{
        &self.url
    }
    pub fn get_cookies(&self) -> &str{
        &self.cookies
    }
}
