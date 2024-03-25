use sqlx::SqlitePool;
use super::{Error, Param};

#[derive(Debug, Clone)]
pub struct Config{
    pub url: String,
    pub port: u16,
    pub salt: String,
    pub pepper: String,
    pub sleep_time: u64,
    pub per_page: i64,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i64,
    pub title: String,
}

impl Config {
    pub async fn load(pool: &SqlitePool) -> Result<Self, Error>{
        Ok(Self{
            url: Param::get_url(pool).await?,
            port: Param::get_port(pool).await?,
            salt: Param::get_salt(pool).await?,
            pepper: Param::get_pepper(pool).await?,
            sleep_time: Param::get_sleep_time(pool).await?,
            per_page: Param::get_per_page(pool).await?,
            jwt_secret: Param::get_jwt_secret(pool).await?,
            jwt_expires_in: Param::get_jwt_expires_in(pool).await?,
            jwt_maxage: Param::get_jwt_maxage(pool).await?,
            title: Param::get_title(pool).await?,
        })
    }

    pub async fn set_url(&mut self, pool: &SqlitePool, url: &str) -> Result<(), Error>{
        self.url = Param::set(pool, "url", url)
            .await?
            .get_value()
            .to_string();
        Ok(())
    }

    pub async fn set_port(&mut self, pool: &SqlitePool, port: u16) -> Result<(), Error>{
        self.port = Param::set(pool, "port", &port.to_string())
            .await?
            .get_value()
            .parse::<u16>()?;
        Ok(())
    }

    pub async fn set_title(&mut self, pool: &SqlitePool, title: &str) -> Result<(), Error>{
        self.title = Param::set(pool, "title", title)
            .await?
            .get_value()
            .to_string();
        Ok(())
    }
}
