use serde::Deserialize;
use actix_web::{
    Responder,
    web::{
        Path,
        Data,
        Query,
        Json,
    },
    get,
    put,
    post,
    delete,
};
use actix_session::Session;
use tracing::{
    info,
    debug,
    error,
};

use super::{
    AppState,
    super::models::{
        CResponse,
        Channel,
        NewChannel,
        UpdateChannel,
    },
};

static FOLDER: &str = "/app/audios";

#[derive(Deserialize)]
struct Page{
    page: Option<i64>,
}

#[derive(Deserialize)]
struct Info{
    channel_id: i64,
}


#[get("/channels/")]
async fn read_with_pagination(
    data: Data<AppState>,
    session: Session,
    page: Query<Page>,
) -> impl Responder{
    info!("read_all");
    let page = page.page.unwrap_or(1);
    let per_page = data.config.per_page;
    match Channel::read_with_pagination(&data.pool, page, per_page).await{
        Ok(channels) => Ok(CResponse::ok(session, channels)),
        Err(mut e) => {
            error!("Error: {e}");
            e.set_session(session);
            Err(e)
        },
    }
}

#[post("/channels/")]
async fn create(
    data: Data<AppState>,
    session: Session,
    channel: Json<NewChannel>,
) -> impl Responder {
    info!("create");
    match Channel::new(&data.pool, channel.into_inner()).await{
            Ok(channel) => Ok(CResponse::ok(session, channel)),
            Err(mut e) => {
                error!("Error: {e}");
                e.set_session(session);
                Err(e)
            },
        }
}

#[put("/channels/")]
async fn update(
    data: Data<AppState>,
    session: Session,
    channel: Json<UpdateChannel>,
) -> impl Responder {
    info!("update");
    match Channel::update(&data.pool, &channel.into_inner()).await{
            Ok(channel) => Ok(CResponse::ok(session, channel)),
            Err(mut e) => {
                error!("Error: {e}");
                e.set_session(session);
                Err(e)
            },
        }
}


#[get("/channels/{channel_id}/")]
async fn read(
    data: Data<AppState>,
    session: Session,
    path: Path<Info>,
) -> impl Responder{
    info!("read");
    match Channel::read(&data.pool, path.channel_id).await{
            Ok(channel) => Ok(CResponse::ok(session, channel)),
            Err(mut e) => {
                error!("Error: {e}");
                e.set_session(session);
                Err(e)
            },
        }
}
#[delete("/channels/")]
async fn delete(
    data: Data<AppState>,
    session: Session,
    path: Query<Info>,
) -> impl Responder{
    info!("delete");
    match Channel::delete(&data.pool, path.channel_id).await{
            Ok(channel) => {
                info!("Remove directory {}/{}", FOLDER, &channel.id);
                match tokio::fs::remove_dir_all(format!("{}/{}", FOLDER, &channel.id))
                    .await {
                    Ok(_) => debug!("Removed directorio {}/{}", FOLDER, &channel.id),
                    Err(e) => error!("Can't remove directory {}/{}: {}", FOLDER, &channel.id, e),
                };
                Ok(CResponse::ok(session, channel))
        },
        Err(mut e) => {
            error!("Error: {e}");
            e.set_session(session);
            Err(e)
            },
    }
}
