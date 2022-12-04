use axum::{
    Router,
    Json,
    Extension,
    extract,
    routing::{get, post},
    response::IntoResponse,
};

use super::error::Error;
use crate::http::ApiContext;
use crate::models::episode::{Episode, NewEpisode};

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/episodes",
            post(create)
            .get(read_all)
            .put(update)
        )
        .route("/api/v1/episodes/:id",
            get(read)
            .delete(delete)
        )
}

async fn create(
    ctx: Extension<ApiContext>,
    extract::Json(episode): extract::Json<NewEpisode>,
) -> impl IntoResponse{
    Episode::create(&ctx.pool, episode.channel_id, &episode.title,
            &episode.description, &episode.yt_id, &episode.link, 
            &episode.published_at, &episode.image, episode.listen)
        .await
        .map_err(|error| Error::Sqlx(error))
        .map(|episode| Json(episode))
}

async fn read(
    ctx: Extension<ApiContext>,
    extract::Path(id): extract::Path<i64>
) -> impl IntoResponse{
    Episode::read(&ctx.pool, id)
        .await
        .map_err(|error| Error::Sqlx(error))
        .map(|episode| Json(episode))
}

async fn read_all(
    ctx: Extension<ApiContext>,
) -> impl IntoResponse{
    Episode::read_all(&ctx.pool)
        .await
        .map_err(|error| Error::Sqlx(error))
        .map(|episodes| Json(episodes))
}

async fn update(
    ctx: Extension<ApiContext>,
    Json(episode): Json<Episode>,
) -> impl IntoResponse{
    Episode::update(&ctx.pool, episode)
        .await
        .map_err(|error| Error::Sqlx(error))
        .map(|episode| Json(episode))
}

async fn delete(
    ctx: Extension<ApiContext>,
    extract::Path(id): extract::Path<i64>,
) -> impl IntoResponse{
    Episode::delete(&ctx.pool, id)
        .await
        .map_err(|error| Error::Sqlx(error))
        .map(|episode| Json(episode))
}
