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
use std::fmt;

use crate::http::{ApiContext, Result};
use super::error::Error;

pub fn router() -> Router {
    Router::new()
        .route("api/v1/channels",
            post(create)
            .get(read_all)
            .put(update)
        )
        .route("api/v1/channels/:id",
            get(read)
            .delete(delete)
        )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: i64,
    pub yt_id: String,
    pub path: String,
    pub title: String,
    pub description: String,
    pub last: DateTime<Utc>,
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.yt_id)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewChannel {
    yt_id: String,
    path: String,
    title: String,
    description: String,
    last: DateTime<Utc>,
}


impl Channel{
    fn from_row(row: SqliteRow) -> Channel{
        Channel {
            id: row.get("id"),
            yt_id: row.get("yt_id"),
            path: row.get("path"),
            title: row.get("title"),
            description: row.get("description"),
            last: row.get("last"),
        }
    }
    pub async fn create(pool: &SqlitePool, yt_id: &str, path: &str,
            title: &str, description: &str, last: &DateTime<Utc>) -> Result<Channel, sqlx::Error>{
        let sql = "INSERT INTO channels (yt_id, path, title, description, last)
                   VALUES ($1, $2, $3, $4, $5) RETURNING * ;";
        query(sql)
            .bind(yt_id)
            .bind(path)
            .bind(title)
            .bind(description)
            .bind(last)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read(pool: &SqlitePool, id: i64) -> Result<Channel, sqlx::Error>{
        let sql = "SELECT * FROM channels WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read_by_path(pool: &SqlitePool, path: &str) -> Result<Channel, sqlx::Error>{
        let sql = "SELECT * FROM channels WHERE path = $1";
        query(sql)
            .bind(path)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    async fn read_all(pool: &SqlitePool) -> Result<Vec<Channel>, sqlx::Error>{
        let sql = "SELECT * FROM channels";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    async fn update(pool: &SqlitePool, channel: Channel)
        ->Result<Channel, sqlx::Error>{
        let sql = "UPDATE channels
            SET
                yt_id = COALESCE($2, yt_id),
                path = COALESCE($3, path),
                title = COALESCE($4, title),
                description = COALESCE($5, description),
                last = COALESCE($6, last)
            WHERE id = $1 RETURNING *;";
        query(sql)
            .bind(channel.id)
            .bind(channel.yt_id)
            .bind(channel.path)
            .bind(channel.title)
            .bind(channel.description)
            .bind(channel.last)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
    async fn delete(pool: &SqlitePool, id: i64) -> Result<Channel, sqlx::Error>{
        let sql = "DELETE FROM channels WHERE id = $1 RETURNING *;";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

}

async fn create(
    ctx: Extension<ApiContext>,
    extract::Json(req): extract::Json<NewChannel>,
) -> impl IntoResponse{
    Channel::create( &ctx.pool, &req.yt_id, &req.path, &req.title,
            &req.description, &req.last)
        .await
        .map_err(|error| Error::Sqlx(error))
        .map(|channel| Json(channel))
}

async fn read(
    ctx: Extension<ApiContext>,
    extract::Path(id): extract::Path<i64>,
) -> impl IntoResponse{
    Channel::read(&ctx.pool, id)
        .await
        .map_err(|error| Error::Sqlx(error))
        .map(|channel| Json(channel))
}

async fn read_all(
    ctx: Extension<ApiContext>,
) -> impl IntoResponse{
    Channel::read_all(&ctx.pool)
        .await
        .map_err(|error| Error::Sqlx(error))
        .map(|channels| Json(channels))
}

async fn update(
    ctx: Extension<ApiContext>,
    Json(channel): Json<Channel>,
) -> impl IntoResponse{
    Channel::update(&ctx.pool, channel)
        .await
        .map_err(|error| Error::Sqlx(error))
        .map(|channel| Json(channel))
}

async fn delete(
    ctx: Extension<ApiContext>,
    extract::Path(id): extract::Path<i64>,
) -> impl IntoResponse{
        Channel::delete(&ctx.pool, id)
            .await
            .map_err(|error| Error::Sqlx(error))
            .map(|channel| Json(channel))
}
