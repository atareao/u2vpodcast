use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};
use chrono::{DateTime, Utc};
use tracing::{info, debug};
use std::collections::HashMap;

// Here my things
use super::Error;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Param{
    id: i64,
    key: String,
    value: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}


impl Param{
    #[allow(dead_code)]
    pub fn get_id(&self) -> i64{
        self.id
    }

    pub fn get_key(&self) -> &str{
        &self.key
    }

    pub async fn get_url(pool: &SqlitePool) -> Result<String, Error>{
        Self::get(pool, "url")
            .await
    }

    pub async fn get_salt(pool: &SqlitePool) -> Result<String, Error>{
        Self::get(pool, "salt")
            .await
    }

    pub async fn get_pepper(pool: &SqlitePool) -> Result<String, Error>{
        Self::get(pool, "pepper")
            .await
    }

    pub async fn get_port(pool: &SqlitePool) -> Result<u16, Error>{
        Self::get(pool, "port")
            .await?
            .parse::<u16>()
            .map_err(|e| e.into())
    }

    pub async fn get_sleep_time(pool: &SqlitePool) -> Result<u64, Error>{
        Self::get(pool, "sleep_time")
            .await?
            .parse::<u64>()
            .map_err(|e| e.into())
    }

    pub async fn get_per_page(pool: &SqlitePool) -> Result<i64, Error>{
        Self::get(pool, "per_page")
            .await?
            .parse::<i64>()
            .map_err(|e| e.into())
    }

    pub async fn get_jwt_secret(pool: &SqlitePool) -> Result<String, Error>{
        Self::get(pool, "jwt_secret")
            .await
    }

    pub async fn get_jwt_expires_in(pool: &SqlitePool) -> Result<String, Error>{
        Self::get(pool, "jwt_expires_in")
            .await
    }

    pub async fn get_jwt_maxage(pool: &SqlitePool) -> Result<i64, Error>{
        Self::get(pool, "jwt_maxage")
            .await?
            .parse::<i64>()
            .map_err(|e| e.into())
    }

    pub async fn get_title(pool: &SqlitePool) -> Result<String, Error>{
        Self::get(pool, "title")
            .await
    }

    pub fn get_value(&self) -> &str{
        &self.value
    }

    fn from_row(row: SqliteRow) -> Self{
        Self{
            id: row.get("id"),
            key: row.get("key"),
            value: row.get("value"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }

    pub async fn get(pool: &SqlitePool, key: &str) -> Result<String, Error>{
        debug!("get {key}");
        let sql = "SELECT value FROM config WHERE key = $1";
        query(sql)
            .bind(key)
            .map(|row: SqliteRow| -> String {row.get(0)})
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_all(pool: &SqlitePool) -> Result<HashMap<String, String>, Error>{
        info!("get_all");
        let sql = "SELECT * FROM config";
        let params = query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await?;
        let mut kv = HashMap::new();
        for param in params{
            debug!("{:?}", param);
            kv.insert(param.key, param.value);
        }
        Ok(kv)
    }

    pub async fn set(pool: &SqlitePool, key: &str, value: &str) -> Result<Param, Error>{
        debug!("set {key}={value}");
        let current_ts = Utc::now();
        let sql = "INSERT INTO config(key, value, updated_at) \
            VALUES($1, $2, $3)
            ON CONFLICT(key) DO UPDATE SET
            value=excluded.value,
            updated_at=excluded.updated_at
            RETURNING *";
        query(sql)
            .bind(key)
            .bind(value)
            .bind(current_ts)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }
}



