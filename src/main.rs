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

mod models;
mod http;
mod config;

#[tokio::main]
async fn main(){
    let configuration = read_configuration().await;

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
    let pool2 = pool.clone();
    tokio::spawn(async move {
        loop {
            do_the_work(&pool2).await;
            tracing::info!("Sleep time: {}", sleep_time);
            tokio::time::sleep(time::Duration::from_secs(sleep_time)).await;
        }
    });
    http::serve(configuration, pool).await.unwrap();
}


async fn do_the_work(pool: &SqlitePool){
    let configuration = &read_configuration().await;
    let folder = configuration.get_folder();
    let cookies = configuration.get_cookies();
    let ytdlp_path = configuration.get_ytdlp_path();

    let ytdlp = Ytdlp::new(ytdlp_path, cookies);
    let now = Utc::now();
    for a_channel in configuration.get_channels().as_slice(){
        let channel_id = &a_channel.get_id();
        tokio::fs::create_dir_all(format!("{}/{}", folder, &a_channel.get_id()))
            .await;
        tracing::info!("Getting new videos for channel: {}", a_channel);
        let last = if Episode::number_of_episodes(pool, channel_id).await > 0{
            Episode::get_max_date(pool, &a_channel.get_id()).await
        }else{
            DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc)
        };
        let days = (now.timestamp() - last.timestamp())/86400;
        tracing::info!("Number of days: {}", days);
        match ytdlp.get_latest(&a_channel.get_url(), days).await{
            Ok(ytvideos) => {
                tracing::info!("Getting {} videos", ytvideos.len());
                for ytvideo in ytvideos{
                    if Episode::exists(pool, channel_id, &ytvideo.id).await{
                        tracing::info!("El video {} titulado '{}', existe", &ytvideo.id, &ytvideo.title);
                        continue;
                    }
                    tracing::info!("Downloading video: {:?}", ytvideo);
                    let filename = format!("{}/{}/{}.mp3", folder, &a_channel.get_id(), &ytvideo.id);
                    let salida = ytdlp.download(&ytvideo.id, &filename).await;
                    if salida.success() {
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
                            Ok(_) => {
                                tracing::info!("Creaded episode: {}", title);
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


async fn read_configuration() -> Configuration{
    let content = match tokio::fs::read_to_string("config.yml")
        .await {
            Ok(value) => value,
            Err(e) => {
                println!("Error with config file `config.yml`: {}",
                    e.to_string());
                process::exit(0);
            }
        };
    match Configuration::new(&content){
        Ok(configuration) => configuration,
        Err(e) => {
            println!("Error with config file `config.yml`: {}",
                e.to_string());
            process::exit(0);
        }
    }
}
