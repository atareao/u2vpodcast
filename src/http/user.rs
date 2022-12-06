use crate::http::ApiContext;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use axum::extract::Extension;
use axum::routing::post;
use axum::{Json, Router};

use crate::http::error;
use crate::http::extractor::AuthUser;
use crate::models::user::User;


pub fn router() -> Router {
    // By having each module responsible for setting up its own routing,
    // it makes the root module a lot cleaner.
    Router::new()
        .route("/api/v1/users",
            post(create_user)
            //.put(update_user)
            )
        .route("/api/v1/users/login", post(login_user))
}

#[derive(serde::Deserialize, Default, PartialEq, Eq)]
struct LoginUpdateNewUser {
    username: String,
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct TokenUser {
    token: String,
    username: String,
}

// https://realworld-docs.netlify.app/docs/specs/backend-specs/endpoints#registration
async fn create_user(
    ctx: Extension<ApiContext>,
    Json(req): Json<LoginUpdateNewUser>,
) -> Result<Json<TokenUser>, error::Error> {
    let username = req.username;
    let hashed_password = hash_password(req.password).await?;

    let user = User::create(&ctx.pool, &username, &hashed_password)
        .await
        .unwrap();

    Ok(Json(TokenUser {
        token: AuthUser { id: user.id }.to_jwt(&ctx),
        username: user.username,
    }))
}

async fn login_user(
    ctx: Extension<ApiContext>,
    Json(req): Json<LoginUpdateNewUser>,
) -> Result<Json<TokenUser>, error::Error> {
    let user = User::search_by_username(&ctx.pool, &req.username)
        .await
        .unwrap();

    verify_password(req.password, user.hashed_password).await?;

    Ok(Json(TokenUser {
            token: AuthUser {
                id: user.id,
            }
            .to_jwt(&ctx),
            username: user.username,
    }))
}

async fn get_current_user(
    auth_user: AuthUser,
    ctx: Extension<ApiContext>,
) -> Result<Json<TokenUser>, error::Error> {
    let user = User::read(&ctx.pool, auth_user.id)
        .await
        .unwrap();

    Ok(Json(TokenUser {
            token: auth_user.to_jwt(&ctx),
            username: user.username,
    }))
}

// https://realworld-docs.netlify.app/docs/specs/backend-specs/endpoints#update-user
// Semantically, because this route allows a partial update it should be `PATCH`, not `PUT`.
// However, we have a spec to follow so `PUT` it is.
async fn update_user(
    auth_user: AuthUser,
    ctx: Extension<ApiContext>,
    Json(req): Json<LoginUpdateNewUser>,
) -> Result<Json<TokenUser>, error::Error> {
    let username = req.username;
    let hashed_password = hash_password(req.password)
        .await
        .unwrap();
    let mut user = User::read(&ctx.pool, auth_user.id)
        .await
        .unwrap();
    if username != user.username || hashed_password != user.hashed_password{
        user.username = username;
        user.hashed_password = hashed_password;
        user = User::update(&ctx.pool, user).await.unwrap();
    }
    Ok(Json(TokenUser {
        token: auth_user.to_jwt(&ctx),
        username: user.username,
    }))
}

async fn hash_password(password: String) -> Result<String, error::Error> {
    // Argon2 hashing is designed to be computationally intensive,
    // so we need to do this on a blocking thread.
    tokio::task::spawn_blocking(move || -> Result<String, error::Error> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(
            PasswordHash::generate(Argon2::default(), password, salt.as_str())
                .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
                .to_string(),
        )
    })
    .await
    .unwrap()
}

async fn verify_password(password: String, password_hash: String) -> Result<(), error::Error> {
    tokio::task::spawn_blocking(move || -> Result<(), error::Error> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => error::Error::Unauthorized,
                _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
            })
    })
    .await
    .unwrap()
}
