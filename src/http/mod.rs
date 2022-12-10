use std::{sync::Arc, net::{SocketAddr, Ipv4Addr}};
use reqwest::header::HeaderName;
use sqlx::SqlitePool;
use axum::{
    Extension,
    Router,
    response::Response,
    middleware::{self, Next},
    http::Request,
    http::header::AUTHORIZATION,
};
use crate::config::Configuration;
use tower_http::trace::TraceLayer;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tera::Tera;

pub mod user;
pub mod channel;
pub mod episode;
pub mod error;
pub mod rss;
pub mod estatic;
pub mod html;
pub mod extractor;
pub use error::{Error, ResultExt};


#[derive(Clone)]
struct ApiContext {
    config: Arc<Configuration>,
    pool: SqlitePool,
}

pub async fn serve(config: Configuration, pool: SqlitePool) -> anyhow::Result<()> {
    let tera = match Tera::new("templates/**/*.html"){
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Error: {}", e);
            std::process::exit(1);
        },
    };

    let app = api_router().layer(
        ServiceBuilder::new()
            .layer(Extension(ApiContext {
                config: Arc::new(config.clone()),
                pool,
            }))
            // Enables logging. Use `RUST_LOG=tower_http=debug`
            .layer(TraceLayer::new_for_http())
            .layer(Extension(tera))
            .layer(CookieManagerLayer::new())
    );

    axum::Server::bind(
        &SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.get_port()))
        .serve(app.into_make_service())
        .await
        .map_err(|err| anyhow::anyhow!("Can't init"))
    
}

fn api_router() -> Router {
    user::router()
        .merge(episode::router())
        .merge(channel::router())
        .merge(rss::router())
        .merge(estatic::router())
        .merge(html::router())
}
