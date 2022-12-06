use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};

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
            username: row.get("username"),
            hashed_password: row.get("hashed_password"),
        }
    }
    pub async fn create(pool: &SqlitePool, username: &str, hashed_password: &str,
            ) -> Result<User, sqlx::Error>{
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
        let sql = "SELECT * FROM users WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn search_by_username(pool: &SqlitePool, username: &str) -> Result<User, sqlx::Error>{
        let sql = "SELECT * FROM users WHERE username = $1";
        query(sql)
            .bind(username)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn update(pool: &SqlitePool, user: User) -> Result<User, sqlx::Error>{
        let sql = "UPDATE users 
                    SET username = COALESCE($1, users.username),
                       hashed_password = COALESCE($2, users.hashed_password)
                    WHERE id = $3
                    RETURNING *;";
        query(sql)
            .bind(user.username)
            .bind(user.hashed_password)
            .bind(user.id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<User, sqlx::Error>{
        let sql = "DELETE from users WHERE id = $1
                   RETURNING *;";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
}
