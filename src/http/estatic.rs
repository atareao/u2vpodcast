use std::sync::Arc;
use axum::{
    Router,
    handler::HandlerWithoutStateExt,
    http::StatusCode,
};
use tower_http::services::ServeDir;
use crate::http::AppState;

pub fn router() -> Router<Arc<AppState>> {
    let audios_server_dir = ServeDir::new("./audios")
        .not_found_service(handle_error.into_service());
    let assets_server_dir = ServeDir::new("./assets")
        .not_found_service(handle_error.into_service());
    Router::new()
        .nest_service("/media", audios_server_dir)
        .nest_service("/assets", assets_server_dir)
}

async fn handle_error() -> (StatusCode, &'static str){
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...: ")
}
