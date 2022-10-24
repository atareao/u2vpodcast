use actix_web;
use actix_web::{App, HttpServer, web::{self, Data}, dev::ServiceRequest,
    middleware::Logger, Error};
use dotenv::dotenv;
use sqlx::SqlitePool;
use std::{thread, time};
use std::{env, path::Path};
use sqlx::{sqlite::SqlitePoolOptions, migrate::{Migrator, MigrateDatabase}};
use env_logger::Env;
use models::{youtube::YouTube, channel::Channel, episode::Episode};
use tokio;

mod models;
mod routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let port = env::var("PORT").expect("PORT not set");
    let sleep_time: u64 = env::var("SLEEP_TIME").unwrap_or("86400".to_string())
        .parse()
        .unwrap();
    let key = std::env::var("YT_KEY").expect("YT_KEY not set");
    let channel_id = std::env::var("YT_CHANNEL").expect("YT_CHANNEL not set");

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

    let pool2 = pool.clone();
    tokio::spawn(async move {
        loop {
            do_the_work(&pool2, &key).await;
            tokio::time::sleep(time::Duration::from_secs(sleep_time)).await;
        }
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .service(routes::main::root)
            .service(routes::main::rss)
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

async fn do_the_work(pool: &SqlitePool, key: &str){
    let yt = YouTube::new(&key);
    let channels = Channel::read_all(&Data::new(pool.clone())).await.unwrap();
    for channel in channels{
        let channel_id = channel.yt_id;
        let after = Some(channel.last.to_string());
        let videos = yt.get_videos(&channel_id, after, None).await;
        for video in &videos{
            println!("{}", video);
        }
    }
}
