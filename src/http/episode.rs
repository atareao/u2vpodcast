use axum::{
    Router,
    Json,
    Extension,
    extract,
    routing::{get, post},
    response::IntoResponse,
};
use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};
use chrono::{DateTime, Utc};

use crate::http::{ApiContext, Result};
use super::error::Error;

pub fn router() -> Router {
    Router::new()
        .route("api/v1/episodes",
            post(create)
            .get(read_all)
            .put(update)
        )
        .route("api/v1/episodes/:id",
            get(read)
            .delete(delete)
        )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    id: i64,
    channel_id: i64,
    pub title: String,
    pub description: String,
    pub yt_id: String,
    pub link: String,
    pub published_at: DateTime<Utc>,
    pub image: String,
    listen: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEpisode {
    channel_id: i64,
    title: String,
    description: String,
    yt_id: String,
    link: String,
    published_at: DateTime<Utc>,
    image: String,
    listen: bool,
}

impl Episode{
    fn from_row(row: SqliteRow) -> Episode{
        Episode{
            id: row.get("id"),
            channel_id: row.get("channel_id"),
            title: row.get("title"),
            description: row.get("description"),
            yt_id: row.get("yt_id"),
            link: row.get("link"),
            published_at: row.get("published_at"),
            image: row.get("image"),
            listen: row.get("listen"),
        }
    }

    pub async fn create(pool: &SqlitePool, channel_id: i64, title: &str,
            description: &str, yt_id: &str, link: &str, 
            published_at: &DateTime<Utc>, image: &str, listen: bool
    ) -> Result<Episode, sqlx::Error>{
        let sql = "INSERT INTO episodes (channel_id, title, description, yt_id,
                   link, published_at, image, listen)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                   RETURNING id, channel_id, title, description, yt_id, link,
                   published_at, image, listen;";
        query(sql)
            .bind(channel_id)
            .bind(title)
            .bind(description)
            .bind(yt_id)
            .bind(link)
            .bind(published_at)
            .bind(image)
            .bind(listen)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read_by_id(pool: &SqlitePool, id: i64) -> Result<Vec<Episode>, sqlx::Error>{
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
    async fn update(pool: &SqlitePool, episode: Episode) -> Result<Episode, sqlx::Error>{
        let sql = "UPDATE episodes SET channel_id = $2, title = $3,
                   description = $4, yt_id = $5, link = $6, published_at = $7,
                   image = $8, listen = $9 FROM episodes WHERE id = $1
                   RETURNING * ;";
        query(sql)
            .bind(episode.id)
            .bind(episode.channel_id)
            .bind(episode.title)
            .bind(episode.description)
            .bind(episode.yt_id)
            .bind(episode.link)
            .bind(episode.published_at)
            .bind(episode.image)
            .bind(episode.listen)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
    async fn delete(pool: &SqlitePool, id: i64) -> Result<Episode, sqlx::Error>{
        let sql = "DELETE from episodes WHERE id = $1
                   RETURNING id, channel_id, title, description, yt_id, link,
                   published_at, image, listen;";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
}

async fn create(
    ctx: Extension<ApiContext>,
    extract::Json(episode): extract::Json<NewEpisode>,
) -> impl IntoResponse{
    Episode::create(&ctx.pool, episode.channel_id, &episode.title,
            &episode.description, &episode.yt_id, &episode.link, 
            &episode.published_at, &episode.image, episode.listen)
        .await
        .map_err(|error| Error::Sqlx(error))
        .map(|episode| Json(episode))
}

async fn read(
    ctx: Extension<ApiContext>,
    extract::Path(id): extract::Path<i64>
) -> impl IntoResponse{
    Episode::read(&ctx.pool, id)
        .await
        .map_err(|error| Error::Sqlx(error))
        .map(|episode| Json(episode))
}

async fn read_all(
    ctx: Extension<ApiContext>,
) -> impl IntoResponse{
    Episode::read_all(&ctx.pool)
        .await
        .map_err(|error| Error::Sqlx(error))
        .map(|episodes| Json(episodes))
}

async fn update(
    ctx: Extension<ApiContext>,
    Json(episode): Json<Episode>,
) -> impl IntoResponse{
    Episode::update(&ctx.pool, episode)
        .await
        .map_err(|error| Error::Sqlx(error))
        .map(|episode| Json(episode))
}

async fn delete(
    ctx: Extension<ApiContext>,
    extract::Path(id): extract::Path<i64>,
) -> impl IntoResponse{
    Episode::delete(&ctx.pool, id)
        .await
        .map_err(|error| Error::Sqlx(error))
        .map(|episode| Json(episode))
}
