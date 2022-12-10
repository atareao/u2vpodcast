use crate::http::{ApiContext, channel};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use axum::{
    Json,
    Router,
    routing::{get, post},
    extract::{Multipart, Extension},
    response::{Html, Response, IntoResponse},
    http::StatusCode,
    body::Empty,
};
use tower_cookies::{Cookie, Cookies};

use std::collections::HashMap;
use crate::{
    http::{error, extractor::AuthUser},
    models::user::User,
};
use tera::{Tera, Context};


pub fn router() -> Router {
    // By having each module responsible for setting up its own routing,
    // it makes the root module a lot cleaner.
    Router::new()
        .route("/api/v1/users",
            post(create_user)
            .get(get_current_user)
            .put(update_user)
            )
        .route("/api/v1/users/login", post(login_user))
        .route("/signup",
            get(get_signup)
            .post(post_signup)
        )
        .route("/logout",
            get(get_logout)
        )
        .route("/login",
            get(get_login)
            .post(post_login)
        )
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
) -> impl IntoResponse{
    let username = req.username;
    let hashed_password = hash_password(req.password)
        .await
        .unwrap();

    User::create(&ctx.pool, &username, &hashed_password)
        .await
        .map_err(|error| error::Error::Sqlx(error))
        .map(|user| Json(TokenUser {
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
        tracing::info!(password);
        tracing::info!(password_hash);
        match PasswordHash::new(&password_hash){
            Ok(hash) => match hash.verify_password(&[&Argon2::default()], password){
                Ok(_) => Ok(()),
                Err(_) => Err(error::Error::InvalidValue), 
            },
            Err(e) => Err(error::Error::InvalidValue),
        }
    })
    .await.unwrap()
}

async fn get_signup(
    ctx: Extension<ApiContext>,
    t: Extension<Tera>
) -> impl IntoResponse{
    let mut context = Context::new();
    context.insert("title", "Signup");
    Html(t.render("signup.html", &context).unwrap())
}


async fn post_signup(
    cookies: Cookies,
    ctx: Extension<ApiContext>,
    multipart: Multipart,
) -> impl IntoResponse {
    let data = parse_multipart(multipart)
        .await
        .map_err(|err| error_page(&err))?;
    if let (Some(username), Some(password), Some(confirm_password)) = (
        data.get("username"),
        data.get("password"),
        data.get("confirm_password"),
    ) {
        if password != confirm_password {
            Err(error_page(&error::Error::PasswordsDoNotMatch))
        }else{
            match hash_password(password.to_string()).await{
                Ok(hashed_password) =>{
                    match User::create(&ctx.pool, username, &hashed_password).await{
                        Ok(user) => {
                            let token = AuthUser {
                                id: user.id,
                            }
                            .to_jwt(&ctx);
                            Ok(set_token(cookies, &token))
                        },
                        Err(e) => Err(error_page(&e)),
                    }
                },
                Err(_) => Err(error_page(&error::Error::InvalidValue))
            }
        }
    } else {
        Err(error_page(&error::Error::MissingDetails))
    }
}

async fn get_login(
    cookies: Cookies,
    ctx: Extension<ApiContext>,
    t: Extension<Tera>
) -> impl IntoResponse{
    let mut context = Context::new();
    context.insert("title", "Login");
    Html(t.render("login.html", &context).unwrap())
}

async fn post_login(
    cookies: Cookies,
    ctx: Extension<ApiContext>,
    multipart: Multipart,
) -> impl IntoResponse{
    let data = parse_multipart(multipart)
        .await.unwrap();
    tracing::info!("{:?}", data);
    if let (Some(username), Some(password)) = (
        data.get("username"),
        data.get("password"),
    ) {
        match User::search_by_username(&ctx.pool, username).await{
            Ok(user) => {
                match verify_password(password.to_string(), user.hashed_password).await{
                    Ok(_) => {
                        let token = AuthUser {
                            id: user.id,
                        }
                        .to_jwt(&ctx);
                        Ok(set_token(cookies, &token))
                    },
                    Err(e) => Err(error_page(&e)),
                }
            },
            Err(e) => Err(error_page(&e))
        }
    } else {
        Err(error_page(&error::Error::MissingDetails))
    }
}

fn set_token(cookies: Cookies, token: &str) -> impl IntoResponse{
    tracing::info!("post_login");
    cookies.add(Cookie::new("ytpodcast", token.to_string()));
    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", "/channels")
        .body(Empty::new())
        .unwrap()
}

async fn get_logout(
    cookies: Cookies,
    //ctx: Extension<ApiContext>,
    //t: Extension<Tera>,
) -> impl IntoResponse{
    tracing::info!("logout");
    cookies.add(Cookie::new("ytpodcast", "".to_string()));
    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", "/channels")
        .body(Empty::new())
        .unwrap()
}

pub(crate) async fn parse_multipart(
    mut multipart: Multipart,
) -> Result<HashMap<String, String>, error::Error> {
    let mut map = HashMap::new();
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_err| error::Error::ReadError)?
    {
        let name = field.name().ok_or(error::Error::NoName)?.to_string();

        let data = field
            .text()
            .await
            .map_err(|_| error::Error::InvalidValue)?;

        map.insert(name, data);
    }
    Ok(map)
}

pub(crate) fn error_page(err: &dyn std::error::Error) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(format!("Err: {}", err))
        .unwrap()
}
