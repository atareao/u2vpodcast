use axum::{
    Router,
    Extension,
    extract,
    routing::get,
    body::{self, boxed, Body, BoxBody, Empty, Full},
    http::{
        StatusCode,
        header::{self, HeaderValue},
        Request,
        Uri,
    },
    response::{Response, IntoResponse},
};
use tower::ServiceExt;
use tower_http::services::ServeDir;
use mime_guess;

use crate::http::ApiContext;


pub fn router(share: &str) -> Router {
    Router::new()
        .route("/media/*path",
            get(file_handler)
        )
}

pub async fn file_handler(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let res = get_static_file(uri.clone()).await?;
    tracing::info!("{:?}", res);
    tracing::info!("{:?}", uri);

    if res.status() == StatusCode::NOT_FOUND {
        match format!("{}.mp3", uri).parse() {
            Ok(uri_mp3) => get_static_file(uri_mp3).await,
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid URI".to_string())),
        }
    } else {
        Ok(res)
    }
}

async fn get_static_file(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    tracing::info!("Uri: {}", &uri);
    tracing::info!("Path: {}", &uri.path());
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // When run normally, the root is the workspace root
    match ServeDir::new("/tmp").oneshot(req).await {
        Ok(res) => {
            Ok(res.map(boxed))
        },
        Err(err) => {
            tracing::info!("{}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", err),
            ))
        },
    }
}


