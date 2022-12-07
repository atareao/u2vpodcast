use axum::{
    Router,
    extract,
    routing::get,
    body::{self, Empty, Full},
    http::{
        StatusCode,
        header::{self, HeaderValue},
    },
    response::{Response, IntoResponse},
};
use mime_guess;
use include_dir::{include_dir, Dir};

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/media");

pub fn router() -> Router {
    Router::new()
        .route("/media/*path",
            get(static_path)
        )
}

async fn static_path(extract::Path(path): extract::Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    match STATIC_DIR.get_file(path) {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
        Some(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(body::boxed(Full::from(file.contents())))
            .unwrap(),
    }
}
