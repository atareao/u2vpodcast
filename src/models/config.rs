use sqlx::SqlitePool;
use super::{Error, Param};

#[derive(Debug, Clone)]
pub struct Config{
    pub title: String,
    pub url: String,
    pub port: u16,
    pub sleep_time: u64,
    pub per_page: i64,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i64,
}

impl Config {
    pub async fn load(pool: &SqlitePool) -> Result<Self, Error>{
        Ok(Self{
            title: Param::get_title(pool).await?,
            url: Param::get_url(pool).await?,
            port: Param::get_port(pool).await?,
            sleep_time: Param::get_sleep_time(pool).await?,
            per_page: Param::get_per_page(pool).await?,
            jwt_secret: Param::get_jwt_secret(pool).await?,
            jwt_expires_in: Param::get_jwt_expires_in(pool).await?,
            jwt_maxage: Param::get_jwt_maxage(pool).await?,
        })
    }
}
