use crate::http::{ApiContext, Result};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use axum::extract::Extension;
use axum::routing::post;
use axum::{Json, Router};

use crate::http::error::{Error, ResultExt};
use crate::http::extractor::AuthUser;

pub fn router() -> Router {
    // By having each module responsible for setting up its own routing,
    // it makes the root module a lot cleaner.
    Router::new()
        .route("/api/v1/users",
            post(create_user)
            .get(get_current_user)
            .put(update_user))
        .route("/api/v1/users/login", post(login_user))
}

/// A wrapper type for all requests/responses from these routes.
#[derive(serde::Serialize, serde::Deserialize)]
struct UserBody<T> {
    user: T,
}

#[derive(serde::Deserialize, Default, PartialEq, Eq)]
#[serde(default)] // fill in any missing fields with `..UpdateUser::default()`
struct LoginUpdateNewUser {
    username: Option<String>,
    password: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct User {
    token: String,
    username: String,
}

// https://realworld-docs.netlify.app/docs/specs/backend-specs/endpoints#registration
async fn create_user(
    ctx: Extension<ApiContext>,
    Json(req): Json<UserBody<LoginUpdateNewUser>>,
) -> Result<Json<UserBody<User>>> {
    let password_hash = hash_password(req.user.password).await?;

    // I personally prefer using queries inline in request handlers as it's easier to understand the
    // query's semantics in the wider context of where it's invoked.
    //
    // Sometimes queries just get too darn big, though. In that case it may be a good idea
    // to move the query to a separate module.
    let id = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"INSERT INTO users (username, hashed_password) VALUES ($1, $2)
            RETURNING id"#,
        req.user.username,
        password_hash
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("user_username_key", |_| {
        Error::unprocessable_entity([("username", "username taken")])
    });

    Ok(Json(UserBody {
        user: User {
            token: AuthUser { id }.to_jwt(&ctx),
            username: req.user.username,
        },
    }))
}

// https://realworld-docs.netlify.app/docs/specs/backend-specs/endpoints#authentication
async fn login_user(
    ctx: Extension<ApiContext>,
    Json(req): Json<UserBody<LoginUpdateNewUser>>,
) -> Result<Json<UserBody<User>>> {
    let user = sqlx::query!(
        r#"SELECT id, username, hashed_password FROM users WHERE username = $1"#,
        req.user.username,
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::unprocessable_entity([("email", "does not exist")]))?;

    verify_password(req.user.password, user.password_hash).await?;

    Ok(Json(UserBody {
        user: User {
            token: AuthUser {
                id: user.id,
            }
            .to_jwt(&ctx),
            username: user.username,
        },
    }))
}

// https://realworld-docs.netlify.app/docs/specs/backend-specs/endpoints#get-current-user
async fn get_current_user(
    auth_user: AuthUser,
    ctx: Extension<ApiContext>,
) -> Result<Json<UserBody<User>>> {
    let user = sqlx::query!(
        r#"SELECT username FROM users WHERE id = $1"#,
        auth_user.id
    )
    .fetch_one(&ctx.db)
    .await?;

    Ok(Json(UserBody {
        user: User {
            // The spec doesn't state whether we're supposed to return the same token we were passed,
            // or generate a new one. Generating a new one is easier the way the code is structured.
            //
            // This has the side-effect of automatically refreshing the session if the frontend
            // updates its token based on this response.
            token: auth_user.to_jwt(&ctx),
            username: user.username,
        },
    }))
}

// https://realworld-docs.netlify.app/docs/specs/backend-specs/endpoints#update-user
// Semantically, because this route allows a partial update it should be `PATCH`, not `PUT`.
// However, we have a spec to follow so `PUT` it is.
async fn update_user(
    auth_user: AuthUser,
    ctx: Extension<ApiContext>,
    Json(req): Json<UserBody<LoginUpdateNewUser>>,
) -> Result<Json<UserBody<User>>> {
    if req.user == LoginUpdateNewUser::default() {
        // If there's no fields to update, these two routes are effectively identical.
        return get_current_user(auth_user, ctx).await;
    }

    // WTB `Option::map_async()`
    let hashed_password = if let Some(password) = req.user.password {
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
        req.user.username,
        hashed_password,
        auth_user.id
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("user_username_key", |_| {
        Error::unprocessable_entity([("username", "username taken")])
    })?;

    Ok(Json(UserBody {
        user: User {
            token: auth_user.to_jwt(&ctx),
            username: user.username,
        },
    }))
}

async fn hash_password(password: String) -> Result<String> {
    // Argon2 hashing is designed to be computationally intensive,
    // so we need to do this on a blocking thread.
    Ok(tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(
            PasswordHash::generate(Argon2::default(), password, salt.as_str())
                .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
                .to_string(),
        )
    })
    .await
    .context("panic in generating password hash")??)
}

async fn verify_password(password: String, password_hash: String) -> Result<()> {
    Ok(tokio::task::spawn_blocking(move || -> Result<()> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => Error::Unauthorized,
                _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
            })
    })
    .await
    .context("panic in verifying password hash")??)
}
