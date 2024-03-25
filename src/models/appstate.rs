use sqlx::{
    Pool,
    Sqlite,
};
use super::config::Config;


#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub pool: Pool<Sqlite>,
}
