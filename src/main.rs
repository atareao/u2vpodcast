use actix_web;
use actix_web::{App, HttpServer, web::{self, Data}, dev::ServiceRequest,
    middleware::Logger, Error};
use dotenv::dotenv;
use std::{env, path::Path};
use sqlx::{sqlite::SqlitePoolOptions, migrate::{Migrator, MigrateDatabase}};
use env_logger::Env;

mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let port = env::var("PORT").expect("PORT not set");

    if !sqlx::Sqlite::database_exists(&db_url).await.unwrap(){
        sqlx::Sqlite::create_database(&db_url).await.unwrap();
    }

    let migrations = if env::var("RUST_ENV") == Ok("production".to_string()){
        std::env::current_exe().unwrap().parent().unwrap().join("migrations")
    }else{
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        Path::new(&crate_dir).join("./migrations")
    };
    println!("{}", &migrations.display());

    let pool = SqlitePoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await
        .expect("Pool failed");

    Migrator::new(migrations)
        .await
        .unwrap()
        .run(&pool)
        .await
        .unwrap();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .service(routes::main::root)
            .service(routes::channels::read)
            .service(routes::channels::read_all)
            .service(routes::channels::create)
            .service(routes::channels::update)
            .service(routes::channels::delete)
            .service(routes::episodes::read)
            .service(routes::episodes::read_all)
            .service(routes::episodes::create)
            .service(routes::episodes::update)
            .service(routes::episodes::delete)
    })
    .workers(2)
    .bind(format!("0.0.0.0:{}", &port))
    .unwrap()
    .run()
    .await
}
