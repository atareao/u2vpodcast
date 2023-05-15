use std::sync::Arc;
use axum::{
    Router,
    Json,
    extract::{State, Path},
    routing,
    response::IntoResponse,
    http::StatusCode,
};

use crate::{ http::{
    AppState,
    error::YTPError,
}, models::channel::{Channel, NewChannel}};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/v1/channels",
            routing::get(read_all)
        )
        .route("/api/v1/channels",
            routing::post(create)
        )
        .route("/api/v1/channels/:id",
            routing::get(read)
        )
        .route("/api/v1/channels",
            routing::put(update)
        )
        .route("/api/v1/channels",
            routing::delete(delete)
        )
}

async fn read(
    State(app_state): State<Arc<AppState>>,
    Path(channel_id): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let channel = Channel::read(&app_state.pool, channel_id).await
        .map_err(|e| {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Database error: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;
    match channel{
        Some(channel) => Ok((StatusCode::OK, Json(serde_json::to_value(channel).unwrap()))),
        None  => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "User with that email already exists",
            });
            Err((StatusCode::CONFLICT, Json(error_response)))
        }
    }
}

async fn read_all(
    State(app_state): State<Arc<AppState>>,
) -> impl IntoResponse{
    match Channel::read_all(&app_state.pool).await{
        Ok(channels) => (StatusCode::OK, Json(channels)).into_response(),
        Err(_) => YTPError::NotFound.into_response(),
    }
}

async fn create(
    State(app_state): State<Arc<AppState>>,
    Json(new_channel): Json<NewChannel>,
) -> impl IntoResponse{
    tracing::info!("Por aquí");
    match Channel::create(&app_state.pool, new_channel).await{
        Ok(channel) => return(StatusCode::OK, Json(channel)).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            YTPError::NotFound.into_response()
        }
    }
}

async fn update(
    State(app_state): State<Arc<AppState>>,
    Json(channel): Json<Channel>,
) -> impl IntoResponse{
    match Channel::update(&app_state.pool, channel).await{
        Ok(channel) => return(StatusCode::OK, Json(channel)).into_response(),
        Err(_) => YTPError::NotFound.into_response()
    }
}

async fn delete(
    State(app_state): State<Arc<AppState>>,
    Path(channel_id): Path<i64>,
) -> impl IntoResponse{
    match Channel::delete(&app_state.pool, channel_id).await{
        Ok(channel) => return(StatusCode::OK, Json(channel)).into_response(),
        Err(_) => YTPError::NotFound.into_response()
    }
}
