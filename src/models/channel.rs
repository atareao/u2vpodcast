use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};
use chrono::{DateTime, Utc};
use std::fmt;

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
    pub yt_id: String,
    pub path: String,
    pub title: String,
    pub description: String,
    pub last: DateTime<Utc>,
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

    pub async fn read_all(pool: &SqlitePool) -> Result<Vec<Channel>, sqlx::Error>{
        let sql = "SELECT * FROM channels";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn update(pool: &SqlitePool, channel: Channel)
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
    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<Channel, sqlx::Error>{
        let sql = "DELETE FROM channels WHERE id = $1 RETURNING *;";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
}
