use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    pub id: i64,
    pub channel_id: i64,
    pub title: String,
    pub description: String,
    pub yt_id: String,
    pub published_at: DateTime<Utc>,
    pub image: String,
    pub listen: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEpisode {
    pub channel_id: i64,
    pub title: String,
    pub description: String,
    pub yt_id: String,
    pub published_at: DateTime<Utc>,
    pub image: String,
    pub listen: bool,
}

impl Episode{
    fn from_row(row: SqliteRow) -> Episode{
        Episode{
            id: row.get("id"),
            channel_id: row.get("channel_id"),
            title: row.get("title"),
            description: row.get("description"),
            yt_id: row.get("yt_id"),
            published_at: row.get("published_at"),
            image: row.get("image"),
            listen: row.get("listen"),
        }
    }

    pub async fn create(pool: &SqlitePool, channel_id: i64, title: &str,
            description: &str, yt_id: &str,  published_at: &DateTime<Utc>,
            image: &str, listen: bool
    ) -> Result<Episode, sqlx::Error>{
        let sql = "INSERT INTO episodes (channel_id, title, description, yt_id,
                   published_at, image, listen)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;";
        query(sql)
            .bind(channel_id)
            .bind(title)
            .bind(description)
            .bind(yt_id)
            .bind(published_at)
            .bind(image)
            .bind(listen)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read(pool: &SqlitePool, id: i64) -> Result<Vec<Episode>, sqlx::Error>{
        let sql = "SELECT * FROM episodes WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn read_all(pool: &SqlitePool) -> Result<Vec<Episode>, sqlx::Error>{
        let sql = "SELECT * FROM episodes";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }
    pub async fn update(pool: &SqlitePool, episode: Episode) -> Result<Episode, sqlx::Error>{
        let sql = "UPDATE episodes SET channel_id = $2, title = $3,
                   description = $4, yt_id = $5, published_at = $6,
                   image = $7, listen = $8 FROM episodes WHERE id = $1
                   RETURNING * ;";
        query(sql)
            .bind(episode.id)
            .bind(episode.channel_id)
            .bind(episode.title)
            .bind(episode.description)
            .bind(episode.yt_id)
            .bind(episode.published_at)
            .bind(episode.image)
            .bind(episode.listen)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<Episode, sqlx::Error>{
        let sql = "DELETE from episodes WHERE id = $1
                   RETURNING id, channel_id, title, description, yt_id,
                   published_at, image, listen;";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
}
