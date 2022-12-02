use axum::{Router, Extension, Json, extract::Path};
use serde::{Serialize, Deserialize};
use sqlx::{sqlite::SqliteRow, query};
use chrono::{DateTime, Utc};

use crate::http::{ApiContext, Result};
use super::extractor::AuthUser;

pub fn router() -> Router {
    Router::new()
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
    async fn create(
        auth_user: AuthUser,
        ctx: Extension<ApiContext>,
        Json(req): Json<NewEpisode>,
    ) -> Result<Json<Episode>>{
        let sql = "INSERT INTO episodes (channel_id, title, description, yt_id,
                   link, published_at, image, listen)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                   RETURNING id, channel_id, title, description, yt_id, link,
                   published_at, image, listen;";
        query(sql)
            .bind(req.channel_id)
            .bind(req.title)
            .bind(req.description)
            .bind(req.yt_id)
            .bind(req.link)
            .bind(req.published_at)
            .bind(req.image)
            .bind(req.listen)
            .map(Self::from_row)
            .fetch_one(&ctx.db)
            .await
    }

    async fn read(
        auth_user: AuthUser,
        ctx: Extension<ApiContext>,
        Path(id): Path<i64>,
    ) -> Result<Json<Episode>>{
        let sql = "SELECT id, channel_id, title, description, yt_id, link,
                   published_at, image, listen FROM episodes WHERE id = $1;";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(&ctx.db)
            .await
    }

    async fn read_all(
        auth_user: AuthUser,
        ctx: Extension<ApiContext>,
    ) -> Result<Json<Vec<Episode>>>{
        let sql = "SELECT id, channel_id, title, description, yt_id, link,
                   published_at, image, listen FROM episodes";
        query(sql)
            .map(Self::from_row)
            .fetch_all(&ctx.db)
            .await
    }

    async fn update(
        auth_user: AuthUser,
        ctx: Extension<ApiContext>,
        Json(req): Json<Episode>,
    ) -> Result<Json<Episode>>{
        let sql = "UPDATE episodes SET channel_id = $2, title = $3,
                   description = $4, yt_id = $5, link = $6,
                   published_at = $7, image = $8, listen = $9 FROM episodes WHERE id = $1
                   RETURNING id, channel_id, title, description, yt_id, link,
                   published_at, image, listen;";
        query(sql)
            .bind(req.episode.id)
            .bind(req.episode.channel_id)
            .bind(req.episode.title)
            .bind(req.episode.description)
            .bind(req.episode.yt_id)
            .bind(req.episode.link)
            .bind(req.episode.published_at)
            .bind(req.episode.image)
            .bind(req.episode.listen)
            .map(Self::from_row)
            .fetch_one(&ctx.db)
            .map(Self::from_row)
            .await
    }

    async fn delete(
        auth_user: AuthUser,
        ctx: Extension<ApiContext>,
        Path(id): Path<i64>,
    ) -> Result<Json<Episode>>{
        let sql = "DELETE from episodes WHERE id = $1
                   RETURNING id, channel_id, title, description, yt_id, link,
                   published_at, image, listen;";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(&ctx.db)
            .await
    }
}

