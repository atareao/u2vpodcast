use axum::{
    Router,
    routing::get_service,
    http::StatusCode,
    response::IntoResponse,
};
use tower_http::services::ServeDir;

pub fn router() -> Router {
    Router::new()
        .nest_service(
            "/media",
            get_service(ServeDir::new("./audios"))
        .handle_error(handle_error))
        .nest_service(
            "/assets",
            get_service(ServeDir::new("./assets"))
        .handle_error(handle_error))
}
async fn handle_error(_err: tokio::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
