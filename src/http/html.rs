use axum::{
    Router,
    Extension,
    routing::get,
    response::{Html, IntoResponse},
};

use tera::{Tera, Context};
use crate::http::ApiContext;

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
    context.insert("title", "Hola");
    context.insert("name", "lorenzo");
    Html(t.render("hello.html", &context).unwrap())
}
