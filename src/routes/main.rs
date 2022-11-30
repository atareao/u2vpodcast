use axum::{response::IntoResponse, http::StatusCode, Json};
use sqlx::sqlite::SqlitePool;
use std::env;
use crate::models::rss::RSS;


pub async fn root() -> impl IntoResponse{
    (StatusCode::OK, "Hola")
}

pub async fn rss(pool: web::Data<SqlitePool>) -> impl IntoResponse{
    let title = env::var("TITLE").unwrap();
    let description = env::var("DESCRIPTION").unwrap();
    let url = env::var("URL").unwrap();
    let rss = RSS::new(&title, &description, &url);
    (StatusCode::OK, Json(rss))
}
