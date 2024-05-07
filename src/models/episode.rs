use serde_json::Value;
use super::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    query,
    sqlite::{SqlitePool, SqliteRow},
    Row,
};
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    pub id: i64,
    pub channel_id: i64,
    pub title: String,
    #[serde(default = "get_default_empty")]
    pub description: String,
    pub yt_id: String,
    pub webpage_url: String,
    pub published_at: DateTime<Utc>,
    pub duration: String,
    #[serde(default = "get_default_empty")]
    pub image: String,
    pub listen: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

fn get_default_empty() -> String {
    "".to_string()
}

impl Episode {
    fn from_row(row: SqliteRow) -> Self {
        info!("from_row");
        Self {
            id: row.get("id"),
            channel_id: row.get("channel_id"),
            title: row.get("title"),
            description: row.get("description"),
            yt_id: row.get("yt_id"),
            webpage_url: row.get("webpage_url"),
            published_at: row.get("published_at"),
            duration: row.get("duration"),
            image: row.get("image"),
            listen: row.get("listen"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }

    pub async fn new(pool: &SqlitePool, channel_id: i64, title: &str,
            description: &str, yt_id: &str, webpage_url: &str,
            published_at: &DateTime<Utc>, duration: &str, image: &str,
            listen: bool) -> Result<Self, Error>{
        info!("new");
        let created_at = Utc::now();
        let updated_at = created_at;
        let mut episode = Self {
            id: -1,
            channel_id,
            title: title.to_string(),
            description: description.to_string(),
            yt_id: yt_id.to_string(),
            webpage_url: webpage_url.to_string(),
            published_at: *published_at,
            duration: duration.to_string(),
            image: image.to_string(),
            listen,
            created_at,
            updated_at,
        };
        episode.save(pool).await
    }

    pub async fn create(
        pool: &SqlitePool,
        episode: &Self,
    ) -> Result<Episode, Error> {
        let sql = "INSERT INTO episodes (channel_id, title, description, yt_id,
                   webpage_url, published_at, duration, image, listen,
                   created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7,
                   $8, $9, $10, $11) RETURNING *;";
        query(sql)
            .bind(episode.channel_id)
            .bind(&episode.title)
            .bind(&episode.description)
            .bind(&episode.yt_id)
            .bind(&episode.webpage_url)
            .bind(episode.published_at)
            .bind(&episode.duration)
            .bind(&episode.image)
            .bind(episode.listen)
            .bind(episode.created_at)
            .bind(episode.updated_at)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn read_episodes_for_channel(pool: &SqlitePool, channel_id: i64) -> Result<Vec<Self>, Error>{
        info!("read_all");
        let sql = "SELECT * FROM episodes WHERE channel_id =$1 ORDER BY published_at DESC";
        query(sql)
            .bind(channel_id)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
            .map_err(|e| e.into())
    }
    pub async fn read_all(pool: &SqlitePool) -> Result<Vec<Self>, Error>{
        info!("read_all");
        let sql = "SELECT * FROM episodes ORDER BY published_at DESC";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn exists(pool: &SqlitePool, channel_id: i64, yt_id: &str) -> bool {
        let sql = "SELECT count(*) FROM episodes WHERE channel_id = $1 AND yt_id = $2";
        match query(sql)
            .bind(channel_id)
            .bind(yt_id)
            .map(|row: SqliteRow| -> i64 { row.get(0) })
            .fetch_one(pool)
            .await
        {
            Ok(value) => value > 0,
            Err(e) => {
                tracing::info!("Error on exists {}", e);
                false
            }
        }
    }

    #[allow(unused)]
    pub async fn count(pool: &SqlitePool, channel_id: i64) -> i64 {
        let sql = "SELECT count(*) FROM episodes WHERE channel_id = $1";
        match query(sql)
            .bind(channel_id)
            .map(|row: SqliteRow| -> i64 { row.get(0) })
            .fetch_one(pool)
            .await
        {
            Ok(value) => value,
            Err(e) => {
                tracing::info!("Error on count {}", e);
                0
            }
        }
    }


    #[allow(unused)]
    pub async fn read_with_pagination(
        pool: &SqlitePool,
        channel_id: i64,
        page: i64,
        per_page: i64,
    ) -> Result<Vec<Episode>, Error> {
        tracing::debug!(
            "Channel: {}. Página: {}. Páginas: {}",
            channel_id,
            page,
            per_page
        );
        let offset = (page - 1) * per_page;
        let sql = "SELECT * FROM episodes
                   WHERE channel_id = $1 ORDER BY published_at DESC
                   LIMIT $2 OFFSET $3";
        query(sql)
            .bind(channel_id)
            .bind(per_page)
            .bind(offset)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn update(pool: &SqlitePool, episode: &Self) -> Result<Self, Error> {
        info!("update");
        let sql = "UPDATE episodes SET channel_id = $2, title = $3,
                   description = $4, yt_id = $5, published_at = $6,
                   duration =$7, image = $8, listen = $9, updated_at = $10
                   FROM episodes WHERE id = $1 RETURNING * ;";
        let updated_at = Utc::now();
        query(sql)
            .bind(episode.id)
            .bind(episode.channel_id)
            .bind(&episode.title)
            .bind(&episode.description)
            .bind(&episode.yt_id)
            .bind(episode.published_at)
            .bind(&episode.duration)
            .bind(&episode.image)
            .bind(episode.listen)
            .bind(updated_at)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn remove(pool: &SqlitePool, id: i64) -> Result<Episode, Error> {
        info!("remove");
        let sql = "DELETE from episodes WHERE id = $1 RETURNING * ;";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }


    pub async fn save(&mut self, pool: &SqlitePool) -> Result<Self, Error>{
        info!("save");
        if self.id > -1 {
            let saved = Self::update(pool, self).await?;
            self.updated_at = saved.updated_at;
            Ok(saved)
        }else{
            let saved = Self::create(pool, self).await?;
            self.id = saved.id;
            Ok(saved)
        }
    }
}

impl From<Episode> for Value {
    fn from(episode: Episode) -> Value{
        episode.into()
    }
}
