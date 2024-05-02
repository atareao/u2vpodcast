mod models;
mod utils;
mod handlers;

use sqlx::{
    sqlite::SqlitePoolOptions,
    migrate::{
        Migrator,
        MigrateDatabase
    }, SqlitePool,
};

use tokio::{
    spawn,
    time::{
        sleep,
        Duration,
    },
};
use std::{
    str::FromStr,
    env::var,
    path::Path,
};
use tracing_subscriber::{
    Layer,
    EnvFilter,
    layer::SubscriberExt,
    util::SubscriberInitExt
};
use tracing::{
    info,
    error,
};

use models::{
    Error,
    Config,
    AppState,
    User,
    Param,
};
use utils::worker::do_the_work;
use actix_files as af;
use actix_session::{
    SessionMiddleware,
    storage::CookieSessionStore,
};
use actix_web::{
    http::header,
    App,
    HttpServer,
    web::Data,
    middleware::Logger,
    cookie::{
        Key,
        SameSite,
    },
};
use actix_cors::Cors;


static DDBB: &str = "u2vpodcast.db";
static MIGRATIONS_DIR: &str = "migrations";

#[actix_web::main]
async fn main() -> Result<(), Error> {

    let format = time::format_description::parse(
        "[year]-[month padding:zero]-[day padding:zero]T[hour]:[minute]:[second]",
    ).expect("Can't parse timer");
    let offset_in_sec = chrono::Local::now()
        .offset()
        .local_minus_utc();
    let time_offset = time::UtcOffset::from_whole_seconds(offset_in_sec).unwrap();

    let timer = tracing_subscriber::fmt::time::OffsetTime::new(time_offset, format);
    let log_level = var("RUST_LOG")
        .unwrap_or("DEBUG".to_string());
    let log_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_timer(timer)
        //.with_thread_names(true)
        .with_filter(EnvFilter::from_str(&log_level).unwrap());

    tracing_subscriber::registry()
        .with(log_layer)
        .init();

    info!("Log level: {log_level}");

    let config = Config::load().await;

    let db_url = if var("RUST_ENV") == Ok("production".to_string()){
        std::env::current_exe()?
            .parent()
            .unwrap()
            .join("db")
            .join(DDBB)
            .to_str()
            .unwrap()
            .to_string()
    }else{
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        Path::new(&crate_dir)
            .join(DDBB)
            .to_str()
            .unwrap()
            .to_string()
    };
    info!("DB url: {db_url}");
    let db_exists = sqlx::Sqlite::database_exists(&db_url).await.unwrap();
    info!("DB exists: {db_exists}");
    if !db_exists{
        sqlx::Sqlite::create_database(&db_url).await.unwrap();
    }

    let migrations = if var("RUST_ENV") == Ok("production".to_string()){
        std::env::current_exe().unwrap().parent().unwrap().join(MIGRATIONS_DIR)
    }else{
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        Path::new(&crate_dir).join(MIGRATIONS_DIR)
    };
    info!("{}", &migrations.display());

    let pool = SqlitePoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await
        .expect("Pool failed");

    Migrator::new(migrations)
        .await?
        .run(&pool)
        .await?;

    let sleep_time = config.sleep_time;
    let port = config.port;

    if !db_exists {
        User::default(&pool, &config.admin_username, &config.admin_password)
            .await
            .expect("Cant create admin user");
    }


    let pool2 = pool.clone();
    spawn(async move{
        //let auth = HttpAuthentication::bearer(validator);
        loop {
            match do_the_work(&pool2, true).await{
                Ok(_) => {},
                Err(e) => {
                    error!("Error doing the work: {e}");
                }
            }
            info!("Sleep time: {}", &sleep_time);
            sleep(Duration::from_secs(sleep_time * 3600)).await;
        }

    });


    let config2 = config.clone();
    HttpServer::new(move || {
        let appstate = AppState{
            config: config2.clone(),
            pool: pool.clone(),
        };
        let data = Data::new(appstate);
        let path = "/app/html";
        let static_files = String::from(path.strip_suffix('/').unwrap_or(path));
        App::new()
            .wrap(Logger::default())
            .wrap(
                if config.production{
                    SessionMiddleware::builder(
                        CookieSessionStore::default(),
                        Key::from(config.secret_key.as_bytes()).clone()
                    )
                    .cookie_http_only(true)
                    .cookie_same_site(SameSite::None)
                    .cookie_secure(true)
                    .build()
                }else{
                    SessionMiddleware::new(
                        CookieSessionStore::default(),
                        Key::from(config.secret_key.as_bytes()).clone()
                    )
                }
            )
            .wrap(
                Cors::default() // allowed_origin return access-control-allow-origin: * by default
                    //.allowed_origin(&url)
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .expose_headers(&[header::CONTENT_DISPOSITION])
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(Data::clone(&data))
            .service(af::Files::new("/media", "./audios"))
            .service(af::Files::new("/app", static_files.clone())
                .index_file("index.html")
                .default_handler(
                    af::NamedFile::open(
                        [static_files.clone(), "index.html".to_string()].join("/"),
                    )
                    .expect("index file should exist"),
                )
            )
            .configure(handlers::config_services)
    })
    .workers(2)
    .bind(("0.0.0.0", port))?
    .run()
    .await
    .map_err(|e| e.into())

}
