mod models;
mod middleware;
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
use actix_web::{
    http::header,
    App,
    HttpServer,
    web::Data,
    middleware::Logger,
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

    let forze: bool = var("FORZE")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false);
    if forze || ! db_exists{
        let _ = init(&pool).await;
    }

    let config = Config::load(&pool).await?;
    let sleep_time = config.sleep_time;
    let port = config.port;

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
                Cors::default() // allowed_origin return access-control-allow-origin: * by default
                    //.allowed_origin(&url)
                    .allow_any_origin()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
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

async fn init(pool: &SqlitePool) -> Result<(), Error>{
    if let Ok(title) = var("TITLE"){
        let _ = Param::set(pool, "title", &title).await?;
    }
    if let Ok(url) = var("URL"){
        let _ = Param::set(pool, "url", &url).await?;
    }
    if let Ok(port) = var("PORT"){
        let _ = Param::set(pool, "port", &port).await?;
    }
    if let Ok(salt) = var("SALT") {
        let _ = Param::set(pool, "salt", &salt).await?;
    }
    if let Ok(pepper) = var("PEPPER") {
        let _ = Param::set(pool, "pepper", &pepper).await?;
    }
    if let Ok(sleep_time) = var("SLEEP_TIME") {
        let _ = Param::set(pool, "sleep_time", &sleep_time).await?;
    }
    if let Ok(per_page) = var("PER_PAGE") {
        let _ = Param::set(pool, "per_page", &per_page).await?;
    }
    if let Ok(jwt_secret) = var("JWT_SECRET") {
        let _ = Param::set(pool, "jwt_secret", &jwt_secret).await?;
    }
    if let Ok(jwt_expires_in) = var("JWT_EXPIRES_IN") {
        let _ = Param::set(pool, "jwt_expires_in", &jwt_expires_in).await?;
    }
    if let Ok(jwt_maxage) = var("JWT_MAXAGE") {
        let _ = Param::set(pool, "jwt_maxage", &jwt_maxage).await?;
    }
    if let Ok(title) = var("TITLE") {
        let _ = Param::set(pool, "title", &title).await?;
    }
    if let Ok(admin_user) = var("ADMIN_USERNAME") {
        if let Ok(admin_pass) = var("ADMIN_PASSWORD") {
            let config = Config::load(pool).await?;
            let _ = User::default(pool, &admin_user, &admin_pass).await;
        }
    }
    Ok(())
}
