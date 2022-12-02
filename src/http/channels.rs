use axum::{Router, Extension, Json, extract::Path};
use serde::{Serialize, Deserialize};
use sqlx::{sqlite::SqliteRow, query};
use chrono::{DateTime, Utc};

use crate::http::{ApiContext, Result};
use super::extractor::AuthUser;
use super::{BodyItem, BodyItems};

pub fn router() -> Router {
    Router::new()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Channel {
    pub id: i64,
    pub yt_id: String,
    title: String,
    pub last: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewChannel {
    yt_id: String,
    title: String,
    last: DateTime<Utc>,
}

impl Channel{
    fn from_row(row: SqliteRow) -> Channel{
        Channel {
            id: row.get("id"),
            yt_id: row.get("yt_id"),
            title: row.get("title"),
            last: row.get("last"),
        }
    }

    async fn create(
        auth_user: AuthUser,
        ctx: Extension<ApiContext>,
        Json(req): Json<BodyItem<NewChannel>>,
    ) -> Result<Json<BodyItem<Channel>>>{
        let sql = "INSERT INTO channels (yt_id, title, last)
                   VALUES ($1, $2, $3) RETURNING id, yt_id, title, last;";
        let channel = query(sql)
            .bind(req.channel.yt_id)
            .bind(req.channel.title)
            .bind(req.channel.last)
            .map(Self::from_row)
            .fetch_one(&ctx.db)
            .await;
        Ok(Json(BodyItem { item: channel, }))
    }

    async fn read(
        auth_user: AuthUser,
        ctx: Extension<ApiContext>,
        Path(id): Path<i64>,
    ) -> Result<Json<BodyItem<Channel>>>{
        let sql = "SELECT id, yt_id, title, last FROM channels WHERE id = $1";
        let channel = query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(&ctx.db)
            .await;
        Ok(Json(BodyItem { item: channel, }))
    }

    async fn read_all(
        auth_user: AuthUser,
        ctx: Extension<ApiContext>,
    ) -> Result<Json<BodyItems<Channel>>>{
        let sql = "SELECT id, yt_id, title, last FROM channels";
        let channels = query(sql)
            .map(Self::from_row)
            .fetch_all(&ctx.db)
            .await;
        Ok(Json(BodyItems { items: channels, }))
    }

    async fn update(
        auth_user: AuthUser,
        ctx: Extension<ApiContext>,
        Json(req): Json<BodyItem<Channel>>,
    ) -> Result<Json<BodyItem<Channel>>>{
        let sql = "UPDATE channels
            SET
                yt_id = COALESCE($2, yt_id),
                title = COALESCE($3, title),
                last = COALESCE($4, last)
            WHERE id = $1 RETURNING id, yt_id, title, last;";
        let channel = query(sql)
            .bind(req.channel.id)
            .bind(req.channel.yt_id)
            .bind(req.channel.title)
            .bind(req.channel.last)
            .map(Self::from_row)
            .fetch_one(&ctx.db)
            .await;
        Ok(Json(BodyItem { item: channel, }))
    }

    async fn delete(
        auth_user: AuthUser,
        ctx: Extension<ApiContext>,
        Path(id): Path<i64>,
    ) -> Result<Json<BodyItem<Channel>>>{
        let sql = "DELETE FROM channels
            WHERE id = $1 RETURNING id, yt_id, title, last;";
        let channel = query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(&ctx.db)
            .await;
        Ok(Json(BodyItem { item: channel, }))
    }
}
