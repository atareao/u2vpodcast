use axum::{
    Router,
    Json,
    Extension,
    extract::{self, Multipart},
    routing::{get, post},
    response::{Html, IntoResponse, Response},
    http::StatusCode,
    body::Empty,
};
use chrono::Utc;

use crate::{
    http::{
        ApiContext,
        error,
        extractor::{
            AuthUser,
            ExtractAuthCookie,
            parse_multipart
        },
    },
    models::channel::{Channel, NewChannel}
};
use tera::{Tera, Context};

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/channels",
            post(create)
            .get(read_all)
            .put(update)
        )
        .route("/api/v1/channels/:id",
            get(read)
            .delete(delete)
        )
        .route("/channels",
            get(read_all_html)
        )
        .route("/channel",
            get(create_get_html)
            .post(create_post_html)
        )
}

async fn create(
    auth_user: AuthUser,
    ctx: Extension<ApiContext>,
    extract::Json(req): extract::Json<NewChannel>,
) -> impl IntoResponse{
    Channel::create( &ctx.pool, &req.url, &req.path, &req.title,
            &req.description, &req.last)
        .await
        .map_err(|error| error::Error::Sqlx(error))
        .map(|channel| Json(channel))
        //.on_db_error(|e| Error::unprocessable_entity([("error", e.to_string())]))
}

async fn read(
    ctx: Extension<ApiContext>,
    extract::Path(id): extract::Path<i64>,
) -> impl IntoResponse{
    Channel::read(&ctx.pool, id)
        .await
        .map_err(|error| error::Error::Sqlx(error))
        .map(|channel| Json(channel))
}

async fn read_all(
    auth_user: AuthUser,
    ctx: Extension<ApiContext>,
) -> impl IntoResponse{
    Channel::read_all(&ctx.pool)
        .await
        .map_err(|error| error::Error::Sqlx(error))
        .map(|channels| Json(channels))
}

async fn update(
    ctx: Extension<ApiContext>,
    Json(channel): Json<Channel>,
) -> impl IntoResponse{
    Channel::update(&ctx.pool, channel)
        .await
        .map_err(|error| error::Error::Sqlx(error))
        .map(|channel| Json(channel))
}

async fn delete(
    ctx: Extension<ApiContext>,
    extract::Path(id): extract::Path<i64>,
) -> impl IntoResponse{
        Channel::delete(&ctx.pool, id)
            .await
            .map_err(|error| error::Error::Sqlx(error))
            .map(|channel| Json(channel))
}

async fn read_all_html(
    auth_cookie: ExtractAuthCookie,
    ctx: Extension<ApiContext>,
    t: Extension<Tera>,
) -> impl IntoResponse{
    tracing::info!("{:?}", auth_cookie);
    tracing::info!("read_all_html");
    match Channel::read_all(&ctx.pool).await{
        Ok(channels) => {
            let mut context = Context::new();
            context.insert("title", "Channels");
            context.insert("channels", &channels);
            //tracing::info!("{:?}", auth_user);
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(t.render("channels.html", &context).unwrap())
                .unwrap())
        },
        Err(e) => Err(error_page(&error::Error::Sqlx(e))),
    }
}

async fn create_get_html(
    _auth_cookie: ExtractAuthCookie,
    ctx: Extension<ApiContext>,
    t: Extension<Tera>,
) -> impl IntoResponse{
    let mut context = Context::new();
    context.insert("title", "New channel");
    Html(t.render("channel.html", &context).unwrap())
}

async fn create_post_html(
    _auth_cookie: ExtractAuthCookie,
    ctx: Extension<ApiContext>,
    multipart: Multipart,
) -> impl IntoResponse{
    let data = parse_multipart(multipart)
        .await.unwrap();
    tracing::info!("{:?}", data);
    if let (Some(path), Some(title), Some(description), Some(url)) = (
        data.get("path"),
        data.get("title"),
        data.get("description"),
        data.get("url"),
    ) {
        let last = Utc::now();
        match Channel::create(&ctx.pool, url, path, title, description, &last).await{
            Ok(channel) => {
                tracing::info!("{:?}", channel);
                Ok(Response::builder()
                    .status(StatusCode::SEE_OTHER)
                    .header("Location", "/channels")
                    .body(Empty::new())
                    .unwrap())
            },
            Err(_e) => Err(error_page(&error::Error::MissingDetails)),
        }
    }else{
        Err(error_page(&error::Error::MissingDetails))
    }
}

pub(crate) fn error_page(err: &dyn std::error::Error) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(format!("Err: {}", err))
        .unwrap()
}
