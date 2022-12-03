use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub hashed_password: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    username: Option<String>,
    password: Option<String>,
}


impl User{
    fn from_row(row: SqliteRow) -> User{
        User {
            id: row.get("id"),
            username: row.get("yt_id"),
            hashed_password: row.get("path"),
        }
    }
    pub async fn create(pool: &SqlitePool, username: &str, password: &str,
            ) -> Result<User, sqlx::Error>{
        let hashed_password = hash_password(password.to_string()).await.unwrap();
        let sql = "INSERT INTO users (username, hashed_password)
                   VALUES ($1, $2) RETURNING * ;";
        query(sql)
            .bind(username)
            .bind(hashed_password)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read(pool: &SqlitePool, id: i64) -> Result<User, sqlx::Error>{
        let sql = "SELECT * FROM channels WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
}

#[derive(thiserror::Error, Debug)]
pub enum HashError {
    /// Return `401 Unauthorized`
    #[error("authentication required")]
    Unauthorized,
    #[error("failed to generate password hash")]
    Failed,
    #[error("invalid password hash")]
    Invalid,
    #[error("invalid password hash")]
    InvalidPassword,
}

async fn hash_password(password: String) -> Result<String, HashError> {
    // Argon2 hashing is designed to be computationally intensive,
    // so we need to do this on a blocking thread.
    tokio::task::spawn_blocking(move || -> Result<String, HashError> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(
            PasswordHash::generate(Argon2::default(), password, salt.as_str())
                .map_err(|e| HashError::Failed).unwrap()
                .to_string(),
        )
    })
    .await
    .map_err(|e| HashError::Failed)?
}

async fn verify_password(password: String, password_hash: String) -> Result<(), HashError> {
    tokio::task::spawn_blocking(move || -> Result<(), HashError> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| HashError::Invalid)?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => HashError::Unauthorized,
                _ => HashError::InvalidPassword,
            })
    })
    .await
    .map_err(|e| HashError::Failed)?
}
