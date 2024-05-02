use serde::Deserialize;
use actix_web::{
    Responder,
    get,
    web::{
        Path,
        Data,
        Query
    },
};
use actix_session::Session;
use tracing::{
    info,
    error,
    debug,
};

use super::{
    AppState,
    super::models::{
        Episode,
        CResponse,
    },
};

#[derive(Deserialize)]
struct Page{
    page: Option<i64>,
}

#[derive(Deserialize)]
struct Info{
    channel_id: i64,
}

#[get("/channels/{channel_id}/episodes/")]
async fn read_with_pagination(
    data: Data<AppState>,
    session: Session,
    params: Query<Page>,
    path: Path<Info>
) -> impl Responder{
    info!("read_api_channels");
    let config = &data.config;
    let per_page = config.per_page;
    let channel_id = path.channel_id;
    let page = params.page.unwrap_or(1);
    match Episode::read_with_pagination(&data.pool, channel_id, page, per_page).await{
        Ok(episodes) => {
            debug!("{:?}", episodes);
            Ok(CResponse::ok(session, episodes))
        },
        Err(e) => {
            error!("{e}");
            Err(e)
        }
    }
}
