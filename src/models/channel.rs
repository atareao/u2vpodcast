use actix_web::web;
use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, Error, query, Row};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    id: i64,
    yt_id: String,
    title: String,
    last: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewChannel {
    yt_id: String,
    title: String,
    last: DateTime<Utc>,
}

impl Channel{
    pub async fn create(pool: &web::Data<SqlitePool>, new: &NewChannel) -> Result<Channel, Error>{
        let sql = "INSERT INTO channels (yt_id, title, last) VALUES ($1, $2, $3);";
        let id = query(sql)
            .bind(&new.yt_id)
            .bind(&new.title)
            .bind(&new.last)
            .execute(pool.get_ref())
            .await?
        .last_insert_rowid();
        Self::read(pool, id).await
    }

    pub async fn read(pool: &web::Data<SqlitePool>, id: i64) -> Result<Channel, Error>{
        let sql = "SELECT id, yt_id, title, last FROM channels WHERE id = $1";
        query(sql)
            .bind(id)
            .map(|row: SqliteRow| Channel{
                id: row.get("id"),
                yt_id: row.get("yt_id"),
                title: row.get("title"),
                last: row.get("last"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn read_all(pool: &web::Data<SqlitePool>) -> Result<Vec<Channel>, Error>{
        let sql = "SELECT id, yt_id, title, last FROM channels";
        
        query(sql)
            .map(|row: SqliteRow| Channel{
                id: row.get("id"),
                yt_id: row.get("yt_id"),
                title: row.get("title"),
                last: row.get("last"),
            })
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn update(pool: &web::Data<SqlitePool>, channel: Channel) -> Result<Channel, Error>{
        let sql = "UPDATE channesl SET yt_id = $2, title = $3,
            last = $4 FROM episodes channels id = $1";
        query(sql)
            .bind(channel.id)
            .bind(channel.yt_id)
            .bind(channel.title)
            .bind(channel.last)
            .execute(pool.get_ref())
            .await?;
        Self::read(pool, channel.id).await
    }

    pub async fn delete(pool: web::Data<SqlitePool>, id: i64) -> Result<Channel, Error>{
        let channel = Self::read(&pool, id).await?;
        let sql = "DELETE from channels WHERE id = $1";
        query(sql)
            .bind(id)
            .execute(pool.get_ref())
            .await?;
        Ok(channel)
    }
}
