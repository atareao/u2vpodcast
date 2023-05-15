use std::sync::Arc;
use axum::{
    Router,
    Json,
    extract::{Path, State},
    routing::get,
    response::IntoResponse,
};

use crate::{
    http::{
        AppState,
        error::YTPError,
    },
    models::episode::Episode
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/v1/episodes",
            get(read_all)
        )
        .route("/api/v1/episodes/:id",
            get(read)
        )
}

async fn read(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>
) -> impl IntoResponse{
    Episode::read(&app_state.pool, id)
        .await
        .map_err(|error| YTPError::Sqlx(error.to_string()))
        .map(|episode| Json(episode))
}

async fn read_all(
    State(app_state): State<Arc<AppState>>,
) -> impl IntoResponse{
    Episode::read_all(&app_state.pool)
        .await
        .map_err(|error| YTPError::Sqlx(error.to_string()))
        .map(|episodes| Json(episodes))
}
