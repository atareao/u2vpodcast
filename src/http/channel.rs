use axum::{
    Router,
    Json,
    Extension,
    extract,
    routing::{get, post},
    response::IntoResponse,
};

use crate::http::ApiContext;
use super::error::{ResultExt, Error};
use crate::models::channel::{Channel, NewChannel};

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
}

async fn create(
    ctx: Extension<ApiContext>,
    extract::Json(req): extract::Json<NewChannel>,
) -> impl IntoResponse{
    Channel::create( &ctx.pool, &req.yt_id, &req.path, &req.title,
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
