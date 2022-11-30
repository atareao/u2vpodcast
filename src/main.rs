use dotenv::dotenv;
use sqlx::SqlitePool;
use std::time;
use std::{env, path::Path};
use sqlx::{sqlite::SqlitePoolOptions, migrate::{Migrator, MigrateDatabase}};
use models::{youtube::YouTube, channel::Channel, episode::{Episode,
    NewEpisode}, ytdlp::Ytdlp};
use tokio;
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use std::net::SocketAddr;
use tower::{BoxError, ServiceBuilder};
use tower_http::{
    auth::RequireAuthorizationLayer, compression::CompressionLayer, limit::RequestBodyLimitLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod models;
mod routes;
mod http;
mod config;

#[tokio::main]
async fn main(){
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "example_key_value_store=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let port: u16 = env::var("PORT").expect("PORT not set")
        .parse()
        .unwrap();
    let sleep_time: u64 = env::var("SLEEP_TIME").unwrap_or("86400".to_string())
        .parse()
        .unwrap();
    let key = std::env::var("YT_KEY").expect("YT_KEY not set");

    let _title = std::env::var("TITLE").expect("TITLE not set");
    let _description = std::env::var("DESCRIPTION").expect("DESCRIPTION not set");
    let _url = std::env::var("URL").expect("URL not set");

    let cookies = std::env::var("COOKIES").expect("COOKIES not set");
    let folder = std::env::var("FOLDER").expect("FOLDER not set");

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


    let pool2 = pool.clone();
    tokio::spawn(async move {
        loop {
            do_the_work(&pool2, &key, &cookies, &folder).await;
            tokio::time::sleep(time::Duration::from_secs(sleep_time)).await;
        }
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let app = Router::new()
        .route("/", get(routes::main::root));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    
    /*

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(dav_server.clone()))
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
    */
}


async fn do_the_work(pool: &SqlitePool, key: &str, cookies: &str, folder: &str){
    let yt = YouTube::new(&key);
    let ytdlp = Ytdlp::new("yt-dlp", cookies);
    let channels = Channel::read_all(&Data::new(pool.clone())).await.unwrap();
    for mut channel in channels{
        let after = Some(channel.last.to_rfc3339_opts(chrono::SecondsFormat::Secs, true));
        let channel_id = channel.id;
        let mut last = channel.last;
        let videos = yt.get_videos(&channel.yt_id, after, None).await;
        for video in &videos{
            let filename = format!("{}/{}.mp3", folder, &video.yt_id);
            let salida = ytdlp.download(&video.yt_id, &filename).await;
            if salida.success() {
                let new = NewEpisode::new(channel_id, &video);
                let episode = Episode::create(&Data::new(pool.clone()), &new).await.unwrap();
                if last < episode.published_at{
                    last = episode.published_at;
                }
            }else{
                break;
            }
        }
        if last != channel.last{
            channel.last = last;
            Channel::update(&Data::new(pool.clone()), channel).await.unwrap();
        }
    }
}
