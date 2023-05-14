use std::{sync::Arc, net::{SocketAddr, Ipv4Addr}};
use sqlx::SqlitePool;
use axum::{
    Router,
    Extension,
};
use crate::config::Configuration;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tera::Tera;

pub mod channel;
pub mod episode;
pub mod error;
pub mod rss;
pub mod estatic;
pub mod root;


#[derive(Clone)]
struct ApiContext {
    config: Arc<Configuration>,
    pool: SqlitePool,
}

pub async fn serve(config: Configuration, pool: SqlitePool) -> anyhow::Result<()> {

    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
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

    );

    axum::Server::bind(
        &SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.get_port()))
        .serve(app.into_make_service())
        .await
        .map_err(|_err| anyhow::anyhow!("Can't init"))
    
}

fn api_router() -> Router {
    channel::router()
        .merge(episode::router())
        .merge(rss::router())
        .merge(estatic::router())
        .merge(root::router())
}
