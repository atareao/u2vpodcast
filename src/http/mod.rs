use std::{sync::Arc, net::{SocketAddr, Ipv4Addr}};
use reqwest::header::HeaderName;
use sqlx::SqlitePool;
use axum::{
    Extension,
    Router,
    response::Response,
    middleware::{self, Next},
    http::{
        Request,
        header::AUTHORIZATION,
        StatusCode
    }
};
use crate::config::Configuration;
use tower_http::trace::TraceLayer;
use tower::ServiceBuilder;
use tower::{Layer, Service};
use std::task::{Context, Poll};

pub mod channel;
pub mod episode;
pub mod error;
pub mod rss;
pub mod estatic;


#[derive(Clone)]
struct ApiContext {
    config: Arc<Configuration>,
    pool: SqlitePool,
}

pub async fn serve(config: Configuration, pool: SqlitePool) -> anyhow::Result<()> {

    let app = api_router().layer(
        ServiceBuilder::new()
            .layer(Extension(ApiContext {
                config: Arc::new(config.clone()),
                pool,
            }))
            // Enables logging. Use `RUST_LOG=tower_http=debug`
            .layer(TraceLayer::new_for_http())
    );

    axum::Server::bind(
        &SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.get_port()))
        .serve(app.into_make_service())
        .await
        .map_err(|err| anyhow::anyhow!("Can't init"))
    
}

fn api_router() -> Router {
    channel::router()
        .merge(episode::router())
        .merge(rss::router())
        .merge(estatic::router())
}

async fn auth<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if authorize_request(auth_header).await {
        // insert the current user into a request extension so the handler can
        // extract it
        //req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
async fn authorize_request(auth_token: &str) -> bool {
    true
}

