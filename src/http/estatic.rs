use axum::{
    Router,
    routing::{get, get_service},
    http::{
        StatusCode,
        header::{self, HeaderValue},
    },
    response::IntoResponse,
};
use tower_http::services::ServeDir;

pub fn router() -> Router {
    Router::new()
        .route("/favicon.ico", get(favicon))
        .nest_service(
            "/media",
            get_service(ServeDir::new("./audios"))
        .handle_error(handle_error))
}
async fn favicon() -> impl IntoResponse {
    let one_pixel_favicon = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mPk+89QDwADvgGOSHzRgAAAAABJRU5ErkJggg==";
    let pixel_favicon = base64::decode(one_pixel_favicon).unwrap();
    ([(header::CONTENT_TYPE, HeaderValue::from_static("image/png"))], pixel_favicon)
}
async fn handle_error(_err: tokio::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
