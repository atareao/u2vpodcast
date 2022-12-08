use axum::{
    Router,
    Json,
    Extension,
    extract,
    routing::{get, post},
    response::IntoResponse,
};
use tera::{Tera, Context};

use crate::http::ApiContext;
use super::{error::Error, extractor::AuthUser};

pub fn router() -> Router {
    Router::new()
        .route("/hello",
            get(hello)
        )
}

async fn hello(
    ctx: Extension<ApiContext>,
    t: Extension<Tera>
) -> impl IntoResponse{
    let mut context = Context::new();
    context.insert("name", "lorenzo");
    t.render("hello.html", &context).unwrap()
}
