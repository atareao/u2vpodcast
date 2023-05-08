use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};
use std::fmt::{self, Display};
use regex::Regex;
use chrono::{DateTime, Utc, NaiveDateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Channel {
    pub id: i64,
    pub url: String,
    pub title: String,
    pub description: String,
    pub image: Option<String>,
    #[serde(default = "get_default_first")]
    pub first: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewChannel {
    pub url: String,
    pub title: String,
    pub description: String,
    pub image: Option<String>,
    pub first: DateTime<Utc>
}

fn get_default_first() -> DateTime<Utc>{
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc)
}

impl Channel{
    fn from_row(row: SqliteRow) -> Channel{
        Channel {
            id: row.get("id"),
            url: row.get("url"),
            title: row.get("title"),
            description: row.get("description"),
            image: row.get("image"),
            first: row.get("first"),
        }
    }

    pub async fn create(pool: &SqlitePool, url: &str, title: &str,
        description: &str, image: &str, first: &DateTime<Utc>)
            -> Result<Channel, sqlx::Error<{
        let sql = "INSERT INTO channels (url, title, description, image, first) 
                   VALUES ($1, $2, $3, $4, $5) RETURNING *;";
        query(sql)
            .bind(url)
            .bind(title)
            .bind(description)
            .bind(image)
            .bind(first)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.url)
    }
}
