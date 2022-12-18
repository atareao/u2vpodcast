use axum::{
    Router,
    Extension,
    routing::get,
    response::{
        IntoResponse,
        Html,
    },
    http::header::{self, HeaderValue}, extract::Path,
};
use tera::{Tera, Context};

use crate::{ http::ApiContext, models::episode::Episode};

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
        .route("/channels/:path",
            get(get_podcast)
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
    let mut channels_with_id = Vec::new();
    for channel in channels{
        channels_with_id.push(channel.get_complete());
    }
    context.insert("channels", &channels_with_id);
    Html(t.render("channels.html", &context).unwrap())
}

async fn get_podcast(
    ctx: Extension<ApiContext>,
    t: Extension<Tera>,
    Path(path): Path<String>,
) -> impl IntoResponse{
    let mut context = Context::new();
    match Episode::read_all_in_channel(&ctx.pool, &path).await{
        Ok(episodes) => {
            let base_url = ctx.config.get_url();
            context.insert("title", &path);
            context.insert("base_url", base_url);
            context.insert("episodes", &episodes);
            Html(t.render("podcast.html", &context).unwrap())
        },
        Err(_) => {
            context.insert("title", "Channels");
            let channels = ctx.config.get_channels();
            context.insert("channels", channels);
            Html(t.render("podcast.html", &context).unwrap())

        }
    }
}
