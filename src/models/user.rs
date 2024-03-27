use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use jsonwebtoken::{EncodingKey, Header};
use sqlx::{
    sqlite::{
        SqlitePool,
        SqliteRow
    },
    query,
    Row,
};
use actix_web::http::StatusCode;
use tracing::{
    info,
    debug,
};
use base64::{
    engine::general_purpose::STANDARD,
    Engine as _,
};

use super::{
    Error,
    role::Role,
    default_datetime,
    config::Config,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User{
    id: i64,
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


#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub role: String,
    pub iat: i64,
    pub exp: i64,
}

impl TokenClaims {
    pub fn generate_token(config: Config, user: &User) -> String {
        let max_age = config.jwt_maxage;
        debug!("Token Max Age: {}", max_age);
        let secret = STANDARD.encode(config.jwt_secret);
        debug!("Secret: {}", secret);

        let now = Utc::now().timestamp_nanos_opt().unwrap() / 1_000_000_000; // nanosecond -> second
        let payload = TokenClaims {
            iat: now,
            exp: now + max_age * 60, //max_age are minutes
            sub: user.name.to_string(),
            role: user.role.to_string(),
        };

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_base64_secret(&secret).unwrap(),
        )
        .unwrap()
    }
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

fn wrap(salt: &str, pepper: &str, word: &str) -> String{
    info!("wrap");
    let composition = format!("{}{}{}", salt, word, pepper);
    format!("{:x}", md5::compute(composition))
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

    pub fn check_password(&self, config: &Config, password: &str) -> bool{
        info!("check_password");
        let salt = &config.salt;
        let pepper = &config.pepper;
        let hashed_password = wrap(salt, pepper, password);
        self.hashed_password == hashed_password
    }

    pub async fn set_password(&mut self, pool: &SqlitePool, config: &Config, password: String) -> Result<Self, Error>{
        info!("set_password");
        let salt = &config.salt;
        let pepper = &config.pepper;
        let hashed_password = wrap(salt, pepper, &password);
        self.hashed_password = hashed_password;
        self.save(pool).await
    }

    pub async fn default(pool: &SqlitePool, config: &Config, name: &str,
        password: &str) -> Result<Self, Error>{
        let new_user = NewUser{
            name: name.to_string(),
            password: password.to_string(),
            role: Role::Admin,
            active: true,
        };
        Self::new(pool, config, new_user).await

    }

    pub async fn new(pool: &SqlitePool, config: &Config, user: NewUser) -> Result<Self, Error>{
        info!("new");
        let salt = &config.salt;
        let pepper = &config.pepper;
        let hashed_password = wrap(salt, pepper, &user.password);
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
        user.save(&pool).await
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
            .bind(&user.active)
            .bind(&user.created_at)
            .bind(&user.updated_at)
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

    pub async fn exists(pool: &SqlitePool, name: &str) -> bool{
        Self::get_by_name(pool, name).await.is_ok()
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


