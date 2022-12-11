use serde::{Serialize, Deserialize};
use serde_yaml::Error;

use crate::models::channel::Channel;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration{
    log_level: String,
    db_url: String,
    port: u16,
    sleep_time: u64,
    username: String,
    password: String,
    with_authentication: bool,
    url: String,
    cookies: String,
    channels: Vec<Channel>
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
    pub fn get_sleep_time(&self) -> u64{
        self.sleep_time
    }
    pub fn get_port(&self) -> u16{
        self.port
    }
    pub fn is_with_authentication(&self) -> bool{
        self.with_authentication
    }
    pub fn get_username(&self) -> &str{
        &self.username
    }
    pub fn get_password(&self) -> &str{
        &self.password
    }
    pub fn get_url(&self) -> &str{
        &self.url
    }
    pub fn get_cookies(&self) -> &str{
        &self.cookies
    }
    pub fn get_channels(&self) -> &Vec<Channel>{
        &self.channels
    }
    pub fn get_channel(&self, id: &str) -> Option<Channel>{
        tracing::info!("Searching: {}", id);
        for channel in self.channels.as_slice(){
            tracing::info!("{} : {}", channel.get_id(), id);
            if channel.get_id() == id{
                return Some(channel.clone())
            }
        }
        None
    }
}
