use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration{
    log_level: String,
    db_url: String,
    port: String,
    username: String,
    password: String,
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
    pub fn get_port(&self) -> &str{
        &self.port
    }
    pub fn get_username(&self) -> &str{
        &&self.username
    }
    pub fn get_password(&self) -> &str{
        &&self.password
    }
}
