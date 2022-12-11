use axum::{
    Router,
    Json,
    Extension,
    extract,
    routing::get,
    response::IntoResponse,
};

use crate::{
    http::{
        ApiContext,
        error::YTPError,
    },
    models::episode::Episode
};

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/episodes",
            get(read_all)
        )
        .route("/api/v1/episodes/:id",
            get(read)
        )
}

async fn read(
    ctx: Extension<ApiContext>,
    extract::Path(id): extract::Path<i64>
) -> impl IntoResponse{
    Episode::read(&ctx.pool, id)
        .await
        .map_err(|error| YTPError::Sqlx(error.to_string()))
        .map(|episode| Json(episode))
}

async fn read_all(
    ctx: Extension<ApiContext>,
) -> impl IntoResponse{
    Episode::read_all(&ctx.pool)
        .await
        .map_err(|error| YTPError::Sqlx(error.to_string()))
        .map(|episodes| Json(episodes))
}
