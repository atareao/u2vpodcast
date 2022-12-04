use crate::http::ApiContext;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use axum::extract::Extension;
use axum::routing::post;
use axum::{Json, Router};

use crate::http::error;
use crate::http::extractor::AuthUser;

use super::ResultExt;

pub fn router() -> Router {
    // By having each module responsible for setting up its own routing,
    // it makes the root module a lot cleaner.
    Router::new()
        .route("/api/v1/users",
            post(create_user))
        .route("/api/v1/users/login", post(login_user))
}

#[derive(serde::Deserialize, Default, PartialEq, Eq)]
struct LoginUpdateNewUser {
    username: String,
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct User {
    token: String,
    username: String,
}

// https://realworld-docs.netlify.app/docs/specs/backend-specs/endpoints#registration
async fn create_user(
    ctx: Extension<ApiContext>,
    Json(req): Json<LoginUpdateNewUser>,
) -> Result<Json<User>, error::Error> {
    let password_hash = hash_password(req.password).await?;

    // I personally prefer using queries inline in request handlers as it's easier to understand the
    // query's semantics in the wider context of where it's invoked.
    //
    // Sometimes queries just get too darn big, though. In that case it may be a good idea
    // to move the query to a separate module.
    let id = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"INSERT INTO users (username, hashed_password) VALUES ($1, $2)
            RETURNING id"#,
        req.username,
        password_hash
    )
    .fetch_one(&ctx.pool)
    .await
    .on_constraint("user_username_key", |_| {
        error::Error::unprocessable_entity([("username", "username taken")])
    })
    .unwrap();

    Ok(Json(User {
        token: AuthUser { id }.to_jwt(&ctx),
        username: req.username,
    }))
}

// https://realworld-docs.netlify.app/docs/specs/backend-specs/endpoints#authentication
async fn login_user(
    ctx: Extension<ApiContext>,
    Json(req): Json<LoginUpdateNewUser>,
) -> Result<Json<User>, error::Error> {
    let user = sqlx::query!(
        r#"SELECT id, username, hashed_password FROM users WHERE username = $1"#,
        req.username,
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or(error::Error::unprocessable_entity([("email", "does not exist")]))?;

    //verify_password(req.user.password, user.password_hash).await?;

    Ok(Json(User {
            token: AuthUser {
                id: user.id,
            }
            .to_jwt(&ctx),
            username: user.username,
    }))
}

// https://realworld-docs.netlify.app/docs/specs/backend-specs/endpoints#get-current-user
async fn get_current_user(
    auth_user: AuthUser,
    ctx: Extension<ApiContext>,
) -> Result<Json<User>, error::Error> {
    let user = sqlx::query!(
        r#"SELECT username FROM users WHERE id = $1"#,
        auth_user.id
    )
    .fetch_one(&ctx.pool)
    .await?;

    Ok(Json(User {
            // The spec doesn't state whether we're supposed to return the same token we were passed,
            // or generate a new one. Generating a new one is easier the way the code is structured.
            //
            // This has the side-effect of automatically refreshing the session if the frontend
            // updates its token based on this response.
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
) -> Result<Json<User>, error::Error> {
    //if req.username == auth_user.id  {
    //    // If there's no fields to update, these two routes are effectively identical.
    //    return get_current_user(auth_user, ctx).await;
    //}

    let hashed_password = if let password = req.password {
        Some(hash_password(password).await?)
    } else {
        None
    };

    let user = sqlx::query!(
        // This is how we do optional updates of fields without needing a separate query for each.
        // language=PostgreSQL
        r#"
            UPDATE users
            SET username = COALESCE($1, users.username),
                hashed_password = COALESCE($2, users.hashed_password)
            WHERE id = $3
            RETURNING username
        "#,
        req.username,
        hashed_password,
        auth_user.id
    )
    .fetch_one(&ctx.pool)
    .await
    .on_constraint("user_username_key", |_| {
        error::Error::unprocessable_entity([("username", "username taken")])
    })?;

    Ok(Json(User {
        token: auth_user.to_jwt(&ctx),
        username: user.username.unwrap(),
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
