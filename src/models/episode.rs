use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};
use chrono::{DateTime, Utc, NaiveDateTime };
use std::time::{UNIX_EPOCH, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    pub id: i64,
    pub channel_id: i64,
    pub title: String,
    pub description: String,
    pub yt_id: String,
    pub published_at: DateTime<Utc>,
    pub duration: String,
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
    pub duration: String,
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
            duration: row.get("duration"),
            image: row.get("image"),
            listen: row.get("listen"),
        }
    }

    pub async fn create(pool: &SqlitePool, channel_id: i64, title: &str,
            description: &str, yt_id: &str,  published_at: &DateTime<Utc>,
            duration: &str, image: &str, listen: bool
    ) -> Result<Episode, sqlx::Error>{
        let sql = "INSERT INTO episodes (channel_id, title, description, yt_id,
                   published_at, duration, image, listen)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;";
        query(sql)
            .bind(channel_id)
            .bind(title)
            .bind(description)
            .bind(yt_id)
            .bind(published_at)
            .bind(get_duration(duration))
            .bind(image)
            .bind(listen)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn number_of_episodes(pool: &SqlitePool, channel_id: i64) -> i64{
        let sql = "SELECT count(*) FROM episodes WHERE channel_id = $1";
        match query(sql)
            .bind(channel_id)
            .map(|row: SqliteRow| -> i64 {row.get(0)})
            .fetch_one(pool)
            .await {
                Ok(value) => value,
                Err(e) => {
                    tracing::info!("Error on exists {}", e);
                    0
                }
            }
    }

    pub async fn exists(pool: &SqlitePool, channel_id: i64, yt_id: &str) -> bool{
        let sql = "SELECT count(*) FROM episodes WHERE channel_id = $1 AND yt_id = $2";
        match query(sql)
            .bind(channel_id)
            .bind(yt_id)
            .map(|row: SqliteRow| -> i64 {row.get(0)})
            .fetch_one(pool)
            .await {
                Ok(value) => value > 0,
                Err(e) => {
                    tracing::info!("Error on exists {}", e);
                    false
                }
            }
    }

    pub async fn read(pool: &SqlitePool, id: i64) -> Result<Vec<Episode>, sqlx::Error>{
        let sql = "SELECT * FROM episodes WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn read_with_pagination_in_channel(pool: &SqlitePool, channel_id: i64, page: i64, per_page: i64) -> Result<Vec<Episode>, sqlx::Error>{
        tracing::debug!("Channel: {}. Página: {}. Páginas: {}", channel_id, page, per_page);
        let offset = (page - 1) * per_page;
        let sql = "SELECT * FROM episodes WHERE channel_id = $1 ORDER BY published_at DESC LIMIT $2 OFFSET $3";
        query(sql)
            .bind(channel_id)
            .bind(per_page)
            .bind(offset)
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
    pub async fn read_all_in_channel(pool: &SqlitePool, channel_id: i64) -> Result<Vec<Episode>, sqlx::Error>{
        let sql = "SELECT * FROM episodes WHERE channel_id = $1";
        query(sql)
            .bind(channel_id)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }
    pub async fn get_max_date(pool: &SqlitePool, channel_id: i64) -> DateTime<Utc>{
        let sql = "SELECT MAX(published_at) as last_date FROM episodes WHERE channel_id = $1";
        match query(sql)
            .bind(channel_id)
            .fetch_one(pool)
            .await{
                Ok(row) => {
                    row.get(0)
                }, 
                Err(e) => {
                    tracing::info!("Not last: {}", e);
                    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc)
                }
            }
    }
    #[allow(dead_code)]
    pub async fn update(pool: &SqlitePool, episode: Episode) -> Result<Episode, sqlx::Error>{
        let sql = "UPDATE episodes SET channel_id = $2, title = $3,
                   description = $4, yt_id = $5, published_at = $6,
                   duration =$7, image = $8, listen = $9
                    FROM episodes WHERE id = $1 RETURNING * ;";
        query(sql)
            .bind(episode.id)
            .bind(episode.channel_id)
            .bind(episode.title)
            .bind(episode.description)
            .bind(episode.yt_id)
            .bind(episode.published_at)
            .bind(episode.duration)
            .bind(episode.image)
            .bind(episode.listen)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
    #[allow(dead_code)]
    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<Episode, sqlx::Error>{
        let sql = "DELETE from episodes WHERE id = $1 RETURNING * ;";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
}

fn get_duration(duration_str: &str) -> String{
    let partes: Vec<i64> = duration_str.split(':').map(|x| x.parse::<i64>().unwrap()).collect();
    let mut valor = 0;
    for (i, parte) in partes.into_iter().rev().enumerate(){
        valor += parte * 60_i64.pow(i.try_into().unwrap());
    }
    // Creates a new SystemTime from the specified number of whole seconds
    let d = UNIX_EPOCH + Duration::from_secs(valor.try_into().unwrap());
    // Create DateTime from SystemTime
    let datetime = DateTime::<Utc>::from(d);
    // Formats the combined date and time with the specified format string.
    datetime.format("%H:%M:%S").to_string()
}
