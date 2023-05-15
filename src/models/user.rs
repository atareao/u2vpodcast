use serde::{Deserialize, Serialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};
use chrono::{DateTime, Utc};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub role: String,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: i64,
    pub email: String,
    pub role: String,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub user: FilteredUser,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

impl User {
    fn from_row(row: SqliteRow) -> User{
        User{
            id: row.get("id"),
            email: row.get("email"),
            password: row.get("password"),
            role: row.get("role"),
            verified: row.get("veified"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
    pub async fn create(pool: &SqlitePool, email: &str, password: &str) -> Result<User, sqlx::Error>{
        let sql = "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING *";
        query(sql)
            .bind(email.to_ascii_lowercase())
            .bind(password)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    } 
    pub async fn read(pool: &SqlitePool, id: i64) -> Result<Option<User>, sqlx::Error>{
        let sql = "SELECT * FROM users WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_optional(pool)
            .await
    }
    pub async fn exists(pool: &SqlitePool, email: &str) -> Result<bool, sqlx::Error>{
        let sql = "SELECT EXISTS(SELECT 1 FROM users WHERE email = $2)";
        query(sql)
            .bind(email.to_ascii_lowercase())
            .map(|row| -> bool {row.get(1)})
            .fetch_one(pool)
            .await
    }
}

impl FilteredUser{
    pub fn from_user(user: &User) -> Self{
        Self {
            id: user.id,
            email: user.email.to_owned(),
            role: user.role.to_owned(),
            verified: user.verified,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
