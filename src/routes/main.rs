use sqlx::sqlite::SqlitePool;
use actix_web::{web, get, HttpResponse, Error, http::StatusCode, error::ErrorBadRequest};
use serde_json::Value;

use crate::models::{response::Response, rss::RSS};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Simple{
    message: String,
}
impl Simple {
    fn new(message: &str) -> Self{
        Self{
            message: message.to_string(),
        }
    }
    fn get_value(&self) -> Value{
        serde_json::to_value(&self).unwrap()
    }
    
}

#[get("/")]
pub async fn root() -> Result<HttpResponse, Error>{
    let value = Simple::new("Rust").get_value();
    let response = Response::new(200, "Ok", value);
    Ok(HttpResponse::build(StatusCode::OK).body(response.get_json_as_string()))
}

#[get("/rss")]
pub async fn rss(pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error>{
    let rss = RSS::new("Titulo", "Enlace", "Descripción");
    rss.get_feed(&pool)
        .await
        .map(|item| HttpResponse::Ok().body(item))
        .map_err(|e| ErrorBadRequest(e))
}
