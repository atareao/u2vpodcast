use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::{
    sqlite::{
        SqlitePool,
        SqliteRow
    },
    query,
    Row,
};
use actix_session::Session;
use actix_web::http::StatusCode;
use tracing::{
    debug,
    info,
};

use crate::utils::token_utils::{verify_password, self};

use super::{
    Error,
    role::Role,
    default_datetime,
    super::utils::{
            USER_ID_KEY,
            USER_NAME_KEY,
            USER_ROLE_KEY,
            USER_ACTIVE_KEY,
    }
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionUser{
    pub id: i64,
    pub name: String,
    pub role: Role,
    pub active: bool,
}


pub fn from_session(session: Session) -> Result<SessionUser, Error> {
    info!("from_session");
    let id = session.get(USER_ID_KEY)
        .map_err(|_| Error::new("Not user_id", &session))?
        .ok_or(Error::new("Not user_id", &session))?;
    let name = session.get(USER_NAME_KEY)
        .map_err(|_| Error::new("Not user_id", &session))?
        .ok_or(Error::new("Not user_id", &session))?;
    let role = session.get(USER_ROLE_KEY)
        .map_err(|_| Error::new("Not user_id", &session))?
        .ok_or(Error::new("Not user_id", &session))?;
    let active = session.get(USER_ACTIVE_KEY)
        .map_err(|_| Error::new("Not user_id", &session))?
        .ok_or(Error::new("Not user_id", &session))?;
    Ok(SessionUser {
        id,
        name,
        role,
        active,
    })
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User{
    pub id: i64,
    pub name: String,
    pub hashed_password: String,
    pub role: Role,
    pub active: bool,
    #[serde(default = "default_datetime")]
    created_at: DateTime<Utc>,
    #[serde(default = "default_datetime")]
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewUser{
    pub name: String,
    pub password: String,
    pub role: Role,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credentials{
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserClaims {
    pub id: i64,
    pub role: Role,
}


#[derive(Debug, Deserialize)]
pub struct UserSchema {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: i64,
    pub name: String,
    pub role: String,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User{
    fn from_row(row: SqliteRow) -> Self{
        info!("from_row");
        Self{
            id: row.get("id"),
            name: row.get("name"),
            hashed_password: row.get("hashed_password"),
            role: row.get("role"),
            active: row.get("active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }

    pub async fn check_password(&self, password: &str) -> bool{
        info!("check_password");
        verify_password(password, &self.hashed_password).await.is_ok()
    }

    pub async fn default(pool: &SqlitePool, name: &str,
        password: &str) -> Result<Self, Error>{
        let new_user = NewUser{
            name: name.to_string(),
            password: password.to_string(),
            role: Role::Admin,
            active: true,
        };
        debug!("{:?}", new_user);
        Self::new(pool, new_user).await

    }

    pub async fn new(pool: &SqlitePool, user: NewUser) -> Result<Self, Error>{
        info!("new");
        let hashed_password = token_utils::hash_password(&user.password).await;
        let created_at = Utc::now();
        let updated_at = created_at;
        let mut user = Self{
            id: -1,
            name: user.name,
            hashed_password,
            role: user.role,
            active: user.active,
            created_at,
            updated_at,
        };
        user.save(pool).await
    }

    pub async fn save(&mut self, pool: &SqlitePool) -> Result<Self, Error>{
        info!("save");
        if self.id > -1 {
            let saved = Self::update(pool, self).await?;
            self.updated_at = saved.updated_at;
            Ok(saved)
        }else{
            let saved = Self::create(pool, self).await?;
            self.id = saved.id;
            Ok(saved)
        }
    }

    pub async fn read(pool: &SqlitePool, id: i64) -> Result<Self, Error>{
        info!("read");
        let sql = "SELECT * FROM users WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| Error::new_with_status_code(&e.to_string(), StatusCode::NOT_FOUND))
    }

    pub async fn create(pool: &SqlitePool, user: &Self) -> Result<Self, Error>{
        info!("create");
        let sql = "INSERT INTO users (name, hashed_password, role, active, created_at,
                   updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *";
        query(sql)
            .bind(&user.name)
            .bind(&user.hashed_password)
            .bind(&user.role)
            .bind(user.active)
            .bind(user.created_at)
            .bind(user.updated_at)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<Self, Error>{
        info!("delete");
        let sql = "DELETE FROM users WHERE id = $1 RETURNING *";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
        .map_err(|e| e.into())
    }

    pub async fn update(pool: &SqlitePool, user: &Self) -> Result<Self, Error>{
        info!("update");
        let updated_at = Utc::now();
        let sql = "UPDATE users SET hashed_password = $1, role = $2,
                   active = $3, updated_at = $4 WHERE id = $5 RETURNING *";
        query(sql)
            .bind(&user.hashed_password)
            .bind(&user.role)
            .bind(user.active)
            .bind(updated_at)
            .bind(user.id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
        .map_err(|e| e.into())
    }

    pub async fn get_by_name(pool: &SqlitePool, name: &str) -> Result<User, Error>{
        let sql = "SELECT * FROM users WHERE name = $1";
        query(sql)
            .bind(name)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn read_with_pagination(
        pool: &SqlitePool,
        page: i64,
        per_page: i64,
    ) -> Result<Vec<User>, Error> {
        tracing::debug!("Página: {page}. Páginas: {per_page}");
        let offset = (page - 1) * per_page;
        let sql = "SELECT * FROM users ORDER BY created_at ASC LIMIT $1 OFFSET $2";
        query(sql)
            .bind(per_page)
            .bind(offset)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
            .map_err(|e| e.into())
    }
}


