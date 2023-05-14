use std::{sync::Arc, net::{SocketAddr, Ipv4Addr}};
use sqlx::SqlitePool;
use axum::{
    Router,
    extract::FromRequestParts,
    middleware::from_extractor,
    http::{
        header,
        StatusCode,
        request::Parts,
    },
    Extension,
    RequestPartsExt,
};
use async_trait::async_trait;
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
        .layer(from_extractor::<RequireAuth>())
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

struct RequireAuth;

#[async_trait]
impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Extension(ctx)= parts.extract::<Extension<ApiContext>>()
            .await
            .unwrap();
        if ctx.config.is_with_authentication(){
            let auth_header = parts
                .headers
                .get(header::AUTHORIZATION)
                .and_then(|value| value.to_str().ok());
            match auth_header {
                Some(auth_header) if token_is_valid(&ctx, auth_header) => {
                    Ok(Self)
                }
                _ => Err(StatusCode::UNAUTHORIZED),
            }
        }else{
            Ok(Self)
        }
    }
}

fn token_is_valid(ctx: &ApiContext, auth_header: &str) -> bool {
    let base = format!("{}:{}", ctx.config.get_username(),
        ctx.config.get_password());
    let token = format!("Basic {}", base64::encode(base));
    auth_header == token
}

