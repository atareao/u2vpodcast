use axum::{
    Router,
    Json,
    Extension,
    extract,
    routing::get,
    response::IntoResponse,
    http::StatusCode,
};

use crate:: http::{
    ApiContext,
    error::YTPError,
};

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/channels",
            get(read_all)
        )
        .route("/api/v1/channels/:id",
            get(read)
        )
}

async fn read(
    ctx: Extension<ApiContext>,
    extract::Path(id): extract::Path<String>,
) -> impl IntoResponse{
    if let Some(channel) = ctx.config.get_channel(&id){
        return (StatusCode::OK, Json(channel)).into_response()
    }
    return YTPError::NotFound.into_response();
}

async fn read_all(
    ctx: Extension<ApiContext>,
) -> impl IntoResponse{
    (StatusCode::OK, Json(ctx.config.get_channels())).into_response()
}
