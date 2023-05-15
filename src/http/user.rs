use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::State,
    Router,
    routing,
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Extension,
    Json
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::OsRng;
use serde_json::json;

use crate::{
    models::{
        user::{LoginUserSchema, RegisterUserSchema, TokenClaims, User},
        response::FilteredUser,
    },
    http::AppState,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/auth/register",
            routing::post(register_user)
        )
}


pub async fn register_user(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<RegisterUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user_exists = User::exists(&app_state.pool, &body.email)
        .await
        .map_err(|e| {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Database error: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;

    if user_exists {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "User with that email already exists",
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Error while hashing password: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string())?;

    let user = User::create(&app_state.pool, &body.email, &hashed_password)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let user_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
        "user": filter_user(&user)
    })});

    Ok(Json(user_response))
}

fn filter_user(user: &User) -> FilteredUser {
    FilteredUser {
        id: user.id,
        email: user.email.to_owned(),
        role: user.role.to_owned(),
        verified: user.verified,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }
}
