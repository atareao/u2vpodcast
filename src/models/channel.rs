use actix_web::web;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, Error, query, Row};
use chrono::{DateTime, NaiveDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    id: i64,
    yt_id: String,
    title: String,
    last: NaiveDateTime,
}

impl Channel{
    pub async fn create(pool: &web::Data<SqlitePool>, body: Value) -> Result<Channel, Error>{
        println!("{}", body);
        let yt_id = body.get("yt_id").unwrap().as_str().unwrap();
        let title = body.get("title").unwrap().as_str().unwrap();
        let last = body.get("last").unwrap().as_str().unwrap();
        let dt = DateTime::parse_from_rfc3339(last).unwrap();
        let ndt = NaiveDateTime::from_timestamp(dt.timestamp(), 0);
        let sql = "INSERT INTO channels (yt_id, title, last) VALUES ($1, $2, $3);";
        let id = query(sql)
            .bind(yt_id)
            .bind(title)
            .bind(ndt)
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

    pub async fn update(pool: &web::Data<SqlitePool>, body: Value) -> Result<Channel, Error>{
        let id = body.get("id").unwrap().as_i64().unwrap();
        let yt_id = body.get("yt_id").unwrap().as_i64().unwrap();
        let title = body.get("title").unwrap().as_str().unwrap();
        let last = body.get("last").unwrap().as_str().unwrap();
        let dt = DateTime::parse_from_rfc3339(last).unwrap();
        let ndt = NaiveDateTime::from_timestamp(dt.timestamp(), 0);
        let sql = "UPDATE channesl SET yt_id = $2, title = $3,
            last = $4 FROM episodes channels id = $1";
        query(sql)
            .bind(id)
            .bind(yt_id)
            .bind(title)
            .bind(ndt)
            .execute(pool.get_ref())
            .await?;
        Self::read(pool, id).await
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
