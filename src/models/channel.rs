use actix_web::http::StatusCode;
use serde_json::Value;
use serde::{
    Serialize,
    Deserialize
};
use std::fmt::{
    self,
    Display
};
use tracing::{
    info,
    debug,
};
use chrono::{
    DateTime,
    Utc,
};
use sqlx::{
    sqlite::{
        SqlitePool,
        SqliteRow
    },
    query,
    Row
};

use super::{
    Error,
    Episode,
    YTInfo,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Channel {
    pub id: i64,
    pub url: String,
    pub title: String,
    pub active: bool,
    pub description: String,
    pub image: String,
    pub first: DateTime<Utc>,
    pub max: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewChannel {
    pub url: String,
    pub active: bool,
    pub first: DateTime<Utc>,
    pub max: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateChannel {
    pub id: i64,
    pub url: String,
    pub active: bool,
    pub first: DateTime<Utc>,
    pub max: i64,
}

impl Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} - {})", self.id, self.url)
    }
}

impl Channel{
    fn from_row(row: SqliteRow) -> Self{
        info!("from_row");
        Self{
            id: row.get("id"),
            url: row.get("url"),
            title: row.get("title"),
            active: row.get("active"),
            description: row.get("description"),
            image: row.get("image"),
            first: row.get("first"),
            max: row.get("max"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }

    pub async fn new(pool: &SqlitePool, channel: NewChannel) -> Result<Self, Error>{
        info!("new");
        let created_at = Utc::now();
        let updated_at = created_at;
        let ytinfo = match YTInfo::new(&channel.url).await{
            Ok(ytinfo) => ytinfo,
            Err(_) => YTInfo::default(),
        };
        let sql = "INSERT INTO channels (url, title, active, description,
                   image, first, max, created_at, updated_at)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;";
        query(sql)
            .bind(&channel.url)
            .bind(&ytinfo.title)
            .bind(channel.active)
            .bind(&ytinfo.description)
            .bind(&ytinfo.image)
            .bind(channel.first)
            .bind(channel.max)
            .bind(created_at)
            .bind(updated_at)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn read(pool: &SqlitePool, id: i64) -> Result<Self, Error>{
        info!("read");
        let sql = "SELECT * FROM channels WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| Error::new_with_status_code(&e.to_string(), StatusCode::NOT_FOUND))
    }

    pub async fn read_all(pool: &SqlitePool) -> Result<Vec<Self>, Error>{
        info!("read_all");
        let sql = "SELECT * FROM channels";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
            .map_err(|e| e.into())
    }

    #[allow(unused)]
    pub async fn read_with_pagination(
        pool: &SqlitePool,
        page: i64,
        per_page: i64,
    ) -> Result<Vec<Channel>, Error> {
        tracing::debug!("Página: {page}. Páginas: {per_page}");
        let offset = (page - 1) * per_page;
        let sql = "SELECT * FROM channels ORDER BY created_at ASC LIMIT $1 OFFSET $2";
        query(sql)
            .bind(per_page)
            .bind(offset)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn update(pool: &SqlitePool, channel: &UpdateChannel) -> Result<Self, Error>{
        info!("update");
        debug!("{:?}", channel);
        let updated_at = Utc::now();
        let sql = "UPDATE channels SET active = $1, first = $2, max = $3,
                   updated_at = $4 WHERE id = $5 RETURNING *";
        query(sql)
            .bind(channel.active)
            .bind(channel.first)
            .bind(channel.max)
            .bind(updated_at)
            .bind(channel.id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<Self, Error>{
        info!("delete");
        let sql = "DELETE FROM channels WHERE id = $1 RETURNING *";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
        .map_err(|e| e.into())
    }

    #[allow(unused)]
    pub async fn number_of_channels(pool: &SqlitePool) -> i64 {
        let sql = "SELECT count(*) FROM channels";
        match query(sql)
            .map(|row: SqliteRow| -> i64 { row.get(0) })
            .fetch_one(pool)
            .await
        {
            Ok(value) => value,
            Err(e) => {
                tracing::info!("Error on exists {}", e);
                0
            }
        }
    }

    pub async fn number_of_episodes(&self, pool: &SqlitePool) -> i64 {
        Self::total(pool, self.id).await
    }

    pub async fn total(pool: &SqlitePool, channel_id: i64) -> i64 {

        let sql = "SELECT count(*) FROM episodes WHERE channel_id = $1";
        match query(sql)
            .bind(channel_id)
            .map(|row: SqliteRow| -> i64 { row.get(0) })
            .fetch_one(pool)
            .await
        {
            Ok(value) => value,
            Err(e) => {
                tracing::info!("Error on exists {}", e);
                0
            }
        }
    }

    pub async fn episode_exists(&self, pool: &SqlitePool, yt_id: &str) -> bool{
        Episode::exists(pool, self.id, yt_id).await
    }

    pub async fn get_max_date(&self, pool: &SqlitePool) -> DateTime<Utc> {
        let sql = "SELECT MAX(published_at) as last_date FROM episodes WHERE channel_id = $1";
        match query(sql).bind(self.id).fetch_one(pool).await {
            Ok(row) => row.get(0),
            Err(e) => {
                tracing::info!("Not last: {}", e);
                Utc::now()
            }
        }
    }
}

impl From<Channel> for Value {
    fn from(channel: Channel) -> Value {
        channel.into()
    }
}
