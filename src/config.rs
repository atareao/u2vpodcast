use serde::{Serialize, Deserialize};
use serde_yaml::Error;

const DEFAULT_SLEEP_TIME: u64 = 1;
const DEFAULT_PER_PAGE: i64 = 10;
const DEFAULT_MAX_AGE: i32 = 60;
const DEFAULT_EXPIRES: &str = "60m";

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
    #[serde(default = "get_default_expires")]
    jwt_expires_in: String,
    #[serde(default = "get_default_maxage")]
    jwt_maxage: i32,
}

fn get_default_dev() -> bool{
    false
}

fn get_default_expires() -> String{
    DEFAULT_EXPIRES.to_string()
}

fn get_default_maxage() -> i32{
    DEFAULT_MAX_AGE
}

fn get_default_sleep_time() -> u64{
    DEFAULT_SLEEP_TIME
}

fn get_default_per_page() -> i64{
    DEFAULT_PER_PAGE
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
    pub fn get_expires(&self) -> &str{
        &self.jwt_expires_in
    }
    pub fn get_secret(&self) -> &str{
        &self.jwt_secret
    }
    pub fn get_maxage(&self) -> &i32{
        &self.jwt_maxage
    }
}
