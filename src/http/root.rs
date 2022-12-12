use axum::{
    Router,
    Extension,
    routing::get,
    response::{
        IntoResponse,
        Html,
    },
    http::header::{self, HeaderValue},
};
use tera::{Tera, Context};

use crate:: http::ApiContext;

pub fn router() -> Router {
    Router::new()
        .route("/favicon.ico",
            get(favicon)
        )
        .route("/",
            get(get_root)
        )
        .route("/channels",
            get(get_channels)
        )
}

async fn favicon() -> impl IntoResponse {
    let one_pixel_favicon = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mPk+89QDwADvgGOSHzRgAAAAABJRU5ErkJggg==";
    let pixel_favicon = base64::decode(one_pixel_favicon).unwrap();
    ([(header::CONTENT_TYPE, HeaderValue::from_static("image/png"))], pixel_favicon)
}

async fn get_root(
    _ctx: Extension<ApiContext>,
    t: Extension<Tera>,
) -> impl IntoResponse{
    let mut context = Context::new();
    context.insert("title", "Title");
    Html(t.render("index.html", &context).unwrap())
}

async fn get_channels(
    ctx: Extension<ApiContext>,
    t: Extension<Tera>,
) -> impl IntoResponse{
    let mut context = Context::new();
    context.insert("title", "Channels");
    let channels = ctx.config.get_channels();
    context.insert("channels", channels);
    Html(t.render("channels.html", &context).unwrap())
}
