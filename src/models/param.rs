use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};
use chrono::{DateTime, Utc};
use tracing::{info, debug};

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
        debug!("get {}", &self.key);
        &self.key
    }

    pub fn get_value(&self) -> &str{
        &self.value
    }

    fn from_row(row: SqliteRow) -> Self{
        info!("from_row");
        Self{
            id: row.get("id"),
            key: row.get("key"),
            value: row.get("value"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
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



