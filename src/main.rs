use sqlx::SqlitePool;
use tracing_subscriber::EnvFilter;
use std::str::FromStr;
use std::time;
use std::{env, path::Path};
use sqlx::{sqlite::SqlitePoolOptions, migrate::{Migrator, MigrateDatabase}};
use tokio;
use std::process;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::config::Configuration;

mod models;
mod http;
mod config;

#[tokio::main]
async fn main(){
    let content = match tokio::fs::read_to_string("config.yml")
        .await {
            Ok(value) => value,
            Err(e) => {
                println!("Error with config file `config.yml`: {}",
                    e.to_string());
                process::exit(0);
            }
        };
    let configuration = Configuration::new(&content)
        .expect("Someting went wrong");


    tracing_subscriber::registry()
        .with(EnvFilter::from_str(configuration.get_log_level()).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();
    let db_url = configuration.get_db_url();
    let port = configuration.get_port();

    if !sqlx::Sqlite::database_exists(db_url).await.unwrap(){
        sqlx::Sqlite::create_database(db_url).await.unwrap();
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
        .connect(db_url)
        .await
        .expect("Pool failed");

    Migrator::new(migrations)
        .await
        .unwrap()
        .run(&pool)
        .await
        .unwrap();


    let configuration2 = configuration.clone();
    let pool2 = pool.clone();
    tokio::spawn(async move {
        loop {
            let key = "";
            let folder = configuration2.get_folder();
            let sleep_time: u64 = configuration2.get_sleep_time().into();
            do_the_work(&pool2, &key, &folder).await;
            tokio::time::sleep(time::Duration::from_secs(sleep_time)).await;
        }
    });
    http::serve(configuration, pool).await.unwrap();
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


async fn do_the_work(pool: &SqlitePool, key: &str, folder: &str){
    /*
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
    */
}
