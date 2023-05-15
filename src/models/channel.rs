use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};
use std::fmt::{self, Display};
use chrono::{DateTime, NaiveDate, Utc, NaiveDateTime};

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
    pub image: String,
    pub first: String
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

    pub fn get_id(&self) -> i64{
        return self.id
    }

    pub fn get_image(&self) -> Option<String>{
        self.image.clone()
    }

    pub fn get_description(&self) -> String{
        self.description.clone()
    }

    pub fn get_title(&self) -> String{
        self.title.clone()
    }

    pub fn get_url(&self) -> String{
        self.url.clone()
    }

    pub fn get_first(&self) -> DateTime<Utc>{
        self.first.clone()
    }

    pub async fn create(pool: &SqlitePool, new_channel: NewChannel)
            -> Result<Channel, sqlx::Error>{
        tracing::info!("Data: {:?}", new_channel);
        let sql = "INSERT INTO channels (url, title, description, image, first) 
                   VALUES ($1, $2, $3, $4, $5) RETURNING *;";
        let datetime = if new_channel.first.is_empty(){
            DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc)
        }else{
            match NaiveDate::parse_from_str(&new_channel.first, "%Y-%d-%m"){
                Ok(nd) => DateTime::<Utc>::from_utc(nd.and_hms(0, 0, 0), Utc),
                Err(_) => DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            }
        };
        query(sql)
            .bind(new_channel.url)
            .bind(new_channel.title)
            .bind(new_channel.description)
            .bind(new_channel.image)
            .bind(datetime)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn number_of_channels(pool: &SqlitePool) -> i64{
        let sql = "SELECT count(*) FROM channels";
        match query(sql)
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

    pub async fn exists(pool: &SqlitePool, url: &str) -> bool{
        let sql = "SELECT count(*) FROM channels WHERE url = $1";
        match query(sql)
            .bind(url)
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

    pub async fn read(pool: &SqlitePool, id: i64) -> Result<Option<Channel>, sqlx::Error>{
        let sql = "SELECT * FROM channels WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_optional(pool)
            .await
    }

    pub async fn read_with_pagination(pool: &SqlitePool, page: i64, per_page: i64) -> Result<Vec<Channel>, sqlx::Error>{
        tracing::debug!("Página: {}. Páginas: {}", page, per_page);
        let offset = (page - 1) * per_page;
        let sql = "SELECT * FROM channels WHERE ORDER BY title ASC LIMIT $2 OFFSET $3";
        query(sql)
            .bind(per_page)
            .bind(offset)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn read_all(pool: &SqlitePool) -> Result<Vec<Channel>, sqlx::Error>{
        let sql = "SELECT * FROM channels";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn update(pool: &SqlitePool, channel: Channel) -> Result<Channel, sqlx::Error>{
        let sql = "UPDATE channels SET url = $2, title = $3,
                   description = $4, image = $5, first = $6
                    FROM channels WHERE id = $1 RETURNING * ;";
        query(sql)
            .bind(channel.id)
            .bind(channel.url)
            .bind(channel.title)
            .bind(channel.description)
            .bind(channel.image)
            .bind(channel.first)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<Channel, sqlx::Error>{
        let sql = "DELETE from channels WHERE id = $1 RETURNING * ;";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}. {}: {}", self.id, self.title, self.url)
    }
}
