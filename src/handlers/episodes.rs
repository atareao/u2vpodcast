use serde::Deserialize;
use actix_web::{
    Responder,
    get,
    web::{
        Path,
        Data,
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
struct Info{
    channel_id: i64,
}

#[get("/channels/{channel_id}/episodes/")]
async fn read_with_pagination(
    data: Data<AppState>,
    session: Session,
    path: Path<Info>
) -> impl Responder{
    info!("read_api_channels");
    let channel_id = path.channel_id;
    match Episode::read_episodes_for_channel(&data.pool, channel_id).await{
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
