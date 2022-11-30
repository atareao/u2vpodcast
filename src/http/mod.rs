use std::sync::Arc;
use sqlx::SqlitePool;
pub mod users;
pub mod error;
pub mod extractor;

#[derive(Clone)]
struct Context {
    config: Arc<Config>,
    db: SqlitePool,
}
