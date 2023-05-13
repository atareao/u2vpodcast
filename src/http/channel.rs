use axum::{
    Router,
    Json,
    Extension,
    extract::Path,
    routing,
    response::IntoResponse,
    http::StatusCode,
};

use crate::{ http::{
    ApiContext,
    error::YTPError,
}, models::channel::{Channel, NewChannel}};

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/channels",
            routing::get(read_all)
        )
        .route("/api/vi/channels",
            routing::post(create)
        )
        .route("/api/v1/channels/:id",
            routing::get(read)
        )
        .route("/api/v1/channels",
            routing::put(update)
        )
        .route("/api/vi/channels",
            routing::delete(delete)
        )
}

async fn read(
    ctx: Extension<ApiContext>,
    Path(id): Path<i64>,
) -> impl IntoResponse{
    if let Some(channel) = ctx.config.get_channel(id){
        return (StatusCode::OK, Json(channel)).into_response()
    }
    return YTPError::NotFound.into_response();
}

async fn read_all(
    ctx: Extension<ApiContext>,
) -> impl IntoResponse{
    match Channel::read_all(&ctx.pool).await{
        Ok(channels) => (StatusCode::OK, Json(channels)).into_response(),
        Err(_) => YTPError::NotFound.into_response(),
    }
}

async fn create(
    ctx: Extension<ApiContext>,
    Json(new_channel): Json<NewChannel>,
) -> impl IntoResponse{
    match Channel::create(&ctx.pool, new_channel).await{
        Ok(channel) => return(StatusCode::OK, Json(channel)).into_response(),
        Err(_) => YTPError::NotFound.into_response()
    }
}

async fn update(
    ctx: Extension<ApiContext>,
    Json(channel): Json<Channel>,
) -> impl IntoResponse{
    match Channel::update(&ctx.pool, channel).await{
        Ok(channel) => return(StatusCode::OK, Json(channel)).into_response(),
        Err(_) => YTPError::NotFound.into_response()
    }
}

async fn delete(
    ctx: Extension<ApiContext>,
    Path(channel_id): Path<i64>,
) -> impl IntoResponse{
    match Channel::delete(&ctx.pool, channel_id).await{
        Ok(channel) => return(StatusCode::OK, Json(channel)).into_response(),
        Err(_) => YTPError::NotFound.into_response()
    }
}
