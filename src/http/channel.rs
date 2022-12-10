use axum::{
    Router,
    Json,
    Extension,
    extract,
    routing::{get, post},
    response::{Html, IntoResponse, Response},
    http::StatusCode,
};

use crate::http::ApiContext;
use super::{error::Error, extractor::{AuthUser, ExtractAuthCookie}};
use crate::models::channel::{Channel, NewChannel};
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
}

async fn create(
    auth_user: AuthUser,
    ctx: Extension<ApiContext>,
    extract::Json(req): extract::Json<NewChannel>,
) -> impl IntoResponse{
    Channel::create( &ctx.pool, &req.url, &req.path, &req.title,
            &req.description, &req.last)
        .await
        .map_err(|error| Error::Sqlx(error))
        .map(|channel| Json(channel))
        //.on_db_error(|e| Error::unprocessable_entity([("error", e.to_string())]))
}

async fn read(
    ctx: Extension<ApiContext>,
    extract::Path(id): extract::Path<i64>,
) -> impl IntoResponse{
    Channel::read(&ctx.pool, id)
        .await
        .map_err(|error| Error::Sqlx(error))
        .map(|channel| Json(channel))
}

async fn read_all(
    auth_user: AuthUser,
    ctx: Extension<ApiContext>,
) -> impl IntoResponse{
    Channel::read_all(&ctx.pool)
        .await
        .map_err(|error| Error::Sqlx(error))
        .map(|channels| Json(channels))
}

async fn update(
    ctx: Extension<ApiContext>,
    Json(channel): Json<Channel>,
) -> impl IntoResponse{
    Channel::update(&ctx.pool, channel)
        .await
        .map_err(|error| Error::Sqlx(error))
        .map(|channel| Json(channel))
}

async fn delete(
    ctx: Extension<ApiContext>,
    extract::Path(id): extract::Path<i64>,
) -> impl IntoResponse{
        Channel::delete(&ctx.pool, id)
            .await
            .map_err(|error| Error::Sqlx(error))
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
        Err(e) => Err(error_page(&Error::Sqlx(e))),
    }
}
pub(crate) fn error_page(err: &dyn std::error::Error) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(format!("Err: {}", err))
        .unwrap()
}
