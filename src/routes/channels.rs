use actix_web::{delete, get, post, put, web, Error, HttpResponse,
    http::StatusCode, error::ErrorNotFound};
use serde_json::Value;
use sqlx::SqlitePool;
use crate::models::channel::Channel;


#[post("/channels")]
pub async fn create(pool: web::Data<SqlitePool>, body: String) -> Result<HttpResponse, Error>{
    let content: Value = serde_json::from_str(&body).unwrap();
    Channel::create(&pool, content)
        .await
        .map(|item| HttpResponse::Ok().json(item))
        .map_err(|e| ErrorNotFound(e))
}

#[get("/channels")]
pub async fn read_all(pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error>{
    Channel::read_all(&pool)
        .await
        .map(|items| HttpResponse::Ok().json(items))
        .map_err(|e| ErrorNotFound(e))
}

#[get("/channels/{id}")]
pub async fn read(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> Result<HttpResponse, Error>{
    let id = path.into_inner();
    Channel::read(&pool, id)
        .await
        .map(|item| HttpResponse::Ok().json(item))
        .map_err(|e| ErrorNotFound(e))
}

#[put("/channels")]
pub async fn update(pool: web::Data<SqlitePool>, body: String) -> Result<HttpResponse, Error>{
    let content: Value = serde_json::from_str(&body).unwrap();
    Channel::update(&pool, content)
        .await
        .map(|item| HttpResponse::Ok().json(item))
        .map_err(|e| ErrorNotFound(e))
}

#[delete("/channels/{id}")]
pub async fn delete(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> Result<HttpResponse, Error>{
    let id = path.into_inner();
    Channel::delete(pool, id)
        .await
        .map(|item| HttpResponse::Ok().json(item))
        .map_err(|e| ErrorNotFound(e))
}

#[cfg(test)]
mod test{
    use dotenv::dotenv;
    use std::{env, path::Path};
    use actix_web::{test, web::Data, App, middleware::Logger};
    use sqlx::{sqlite::SqlitePoolOptions, migrate::Migrator};
    use env_logger::Env;
    use crate::routes;


    #[actix_web::test]
    async fn test_get_channel(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let migrations = Path::new(&crate_dir).join("./migrations");
        env_logger::init_from_env(Env::default().default_filter_or("info"));
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&db_url)
            .await
            .expect("Pool failed");

        Migrator::new(migrations)
            .await
            .unwrap()
            .run(&pool)
            .await
            .unwrap();

        let app = test::init_service(
            App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(routes::main::root)
            .service(routes::channels::read_all)
        )
        .await;
        let req = test::TestRequest::get().uri("/channels").to_request();
        let result = test::call_and_read_body(&app, req).await;
        println!("{:?}", result);
    }
}
