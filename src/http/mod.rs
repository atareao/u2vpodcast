use std::{sync::Arc, net::{SocketAddr, Ipv4Addr}};
use sqlx::SqlitePool;
use axum::{
    Router,
    Extension,
};
use crate::config::Configuration;
use tower_http::trace::TraceLayer;
use tera::Tera;

pub mod channel;
pub mod episode;
pub mod error;
pub mod rss;
pub mod estatic;
pub mod root;
pub mod jwt_auth;
pub mod user;


#[derive(Clone)]
pub struct AppState {
    pub config: Configuration,
    pub pool: SqlitePool,
}

pub async fn serve(config: Configuration, pool: SqlitePool) -> anyhow::Result<()> {

    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let app = api_router(
            AppState {
                config: config.clone(),
                pool: pool.clone(),
            })
            .layer(TraceLayer::new_for_http())
            .layer(Extension(tera));
    axum::Server::bind(
        &SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.get_port()))
        .serve(app.into_make_service())
        .await
        .map_err(|_err| anyhow::anyhow!("Can't init"))
    
}

fn api_router(app_state: AppState) -> Router {
    channel::router()
        .merge(episode::router())
        .merge(rss::router())
        .merge(estatic::router())
        .merge(root::router())
        .with_state(Arc::new(app_state))
}

