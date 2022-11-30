use std::sync::Arc;
use sqlx::SqlitePool;
use axum::{Extension, Router};
use crate::config::Configuration;
use tower_http::trace::TraceLayer;
use tower::ServiceBuilder;

// Utility modules.

/// Defines a common error type to use for all request handlers, compliant with the Realworld spec.
pub mod users;
pub mod error;
/// Contains definitions for application-specific parameters to handler functions,
/// such as `AuthUser` which checks for the `Authorization: Token <token>` header in the request,
/// verifies `<token>` as a JWT and checks the signature,
/// then deserializes the information it contains.
pub mod extractor;

#[derive(Clone)]
struct Context {
    config: Arc<Configuration>,
    db: SqlitePool,
}

pub async fn serve(config: Configuration, db: SqlitePool) -> anyhow::Result<()> {
    // Bootstrapping an API is both more intuitive with Axum than Actix-web but also
    // a bit more confusing at the same time.
    //
    // Coming from Actix-web, I would expect to pass the router into `ServiceBuilder` and not
    // the other way around.
    //
    // It does look nicer than the mess of `move || {}` closures you have to do with Actix-web,
    // which, I suspect, largely has to do with how it manages its own worker threads instead of
    // letting Tokio do it.
    let app = api_router().layer(
        ServiceBuilder::new()
            .layer(Extension(Context {
                config: Arc::new(config),
                db,
            }))
            // Enables logging. Use `RUST_LOG=tower_http=debug`
            .layer(TraceLayer::new_for_http()),
    );

    // We use 8080 as our default HTTP server port, it's pretty easy to remember.
    //
    // Note that any port below 1024 needs superuser privileges to bind on Linux,
    // so 80 isn't usually used as a default for that reason.
    axum::Server::bind(format!("0.0.0.0:{}", config.get_port()))
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn api_router() -> Router {
    // This is the order that the modules were authored in.
    users::router()
        .merge(profiles::router())
        .merge(articles::router())
}
