use http::episode;
use models::episode::Episode;
use models::ytdlp::Ytdlp;
use sqlx::SqlitePool;
use tracing_subscriber::EnvFilter;
use std::str::FromStr;
use std::time;
use std::{env, path::Path};
use sqlx::{sqlite::SqlitePoolOptions, migrate::{Migrator, MigrateDatabase}};
use tokio;
use chrono::{DateTime, Utc, naive::{NaiveDate, NaiveDateTime}};
use std::process;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::config::Configuration;
use crate::models::channel::Channel;

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


    let sleep_time: u64 = configuration.get_sleep_time().into();
    let folder = configuration.get_folder().to_string();
    let cookies = configuration.get_cookies().to_string();
    let ytdlp_path = configuration.get_ytdlp_path().to_owned();
    let pool2 = pool.clone();
    tokio::spawn(async move {
        loop {
            do_the_work(&pool2, &ytdlp_path, &folder, &cookies).await;
            tokio::time::sleep(time::Duration::from_secs(sleep_time)).await;
        }
    });
    http::serve(configuration, pool).await.unwrap();
}


async fn do_the_work(pool: &SqlitePool, ytdlp_path: &str, folder: &str, cookies: &str){
    let ytdlp = Ytdlp::new(ytdlp_path, cookies);
    let channels = Channel::read_all(pool).await.unwrap();
    let now = Utc::now();
    for a_channel in channels{
        tokio::fs::create_dir_all(format!("{}/{}", folder, &a_channel.path))
            .await;
        tracing::info!("Getting new videos for channel: {}", a_channel);
        let days = (now.timestamp() - a_channel.last.timestamp())/86400;
        tracing::info!("Number of days: {}", days);
        match ytdlp.get_latest(&a_channel.url, days).await{
            Ok(ytvideos) => {
                tracing::info!("Getting {} videos", ytvideos.len());
                for ytvideo in ytvideos{
                    tracing::info!("Downloading video: {:?}", ytvideo);
                    let filename = format!("{}/{}/{}.mp3", folder, &a_channel.path, &ytvideo.id);
                    let salida = ytdlp.download(&ytvideo.id, &filename).await;
                    if salida.success() {
                        let channel_id = a_channel.id;
                        let title = &ytvideo.title;
                        let description = &ytvideo.description;
                        let yt_id = &ytvideo.id;
                        tracing::info!("{}", &ytvideo.upload_date);
                        let published_at = parse_date(&ytvideo.upload_date);
                        let image = &ytvideo.thumbnail;
                        let listen = false;
                        match Episode::create(pool, channel_id, title,
                                description, yt_id, &published_at, image,
                                listen).await{
                            Ok(episode) => {
                                let mut n_channel = a_channel.clone();
                                n_channel.last = published_at;
                                tracing::info!("saved {}", episode.yt_id);
                                match Channel::update(pool, n_channel).await{
                                    Ok(u_channel) => {
                                        tracing::info!("update channel: {}", u_channel);
                                    },
                                    Err(e) => {
                                        tracing::error!("{}", e);
                                        break;
                                    }
                                }
                            },
                            Err(e) => {
                                tracing::error!("{}", e);
                                break;
                            }
                        }
                    }else{
                        tracing::error!("Cant download file {}", filename);
                        break;
                    }
                }
            },
            Err(e) => tracing::error!("{}", e),
        }
    }
}

fn parse_date(date: &str) -> DateTime<Utc>{
    let format = "%Y%m%d";
    let naive_date = NaiveDate::parse_from_str(date, format).unwrap();
    // Add some default time to convert it into a NaiveDateTime
    let naive_datetime: NaiveDateTime = naive_date.and_hms(0,0,0);
    // Add a timezone to the object to convert it into a DateTime<UTC>
    DateTime::<Utc>::from_utc(naive_datetime, Utc)
}
