use serde::Deserialize;
use actix_web::{
    Responder,
    HttpResponse,
    web::{
        self,
        Path,
        Data,
        Query,
        Json,
        ServiceConfig,
    },
    http::StatusCode,
    get,
    patch,
    post,
    delete,
};
use tracing::{
    info,
    debug,
    error,
};
use minijinja::context;

use super::{
    ENV,
    AppState,
    episodes::read_api_episodes,
    super::models::{
        CustomResponse,
        Channel,
        NewChannel,
        UpdateChannel,
    },
};

static FOLDER: &str = "/app/audios";

pub fn api_channels(cfg: &mut ServiceConfig){
    cfg.service(
        web::scope("channels")
            .service(create)
            .service(read)
            .service(read_with_pagination)
            .service(update)
            .service(delete)
            .service(read_api_episodes)
    );
}

pub fn config_channels(cfg: &mut ServiceConfig){
    cfg.service(
        web::resource("channels/")
            .route(web::get().to(read_config_channels))
    );
}

pub fn web_channels(cfg: &mut ServiceConfig){
    cfg.service(
        web::resource("channels/")
            .route(web::get().to(read_web_channels))
    );
}

#[derive(Deserialize)]
struct Page{
    page: Option<i64>,
}

#[derive(Deserialize)]
struct Info{
    channel_id: i64,
}


#[get("/")]
async fn read_with_pagination(
    data: Data<AppState>,
    page: Query<Page>,
) -> impl Responder{
    info!("read_all");
    let page = page.page.unwrap_or(1);
    let per_page = data.config.per_page;
    match Channel::read_with_pagination(&data.pool, page, per_page).await{
        Ok(channel) => Ok(Json(channel)),
        Err(e) => Err(e),
    }
}

#[post("/")]
async fn create(
    data: Data<AppState>,
    channel: Json<NewChannel>,
) -> impl Responder {
    info!("create");
    match Channel::new(&data.pool, channel.into_inner()).await{
            Ok(channel) => Ok(Json(CustomResponse::new(
            StatusCode::OK,
            "Ok",
            channel,
        ))),
            Err(e) => Err(e),
        }
}

#[patch("/")]
async fn update(
    data: Data<AppState>,
    channel: Json<UpdateChannel>,
) -> impl Responder {
    info!("create");
    match Channel::update(&data.pool, &channel.into_inner()).await{
            Ok(channel) => Ok(Json(CustomResponse::new(
            StatusCode::OK,
            "Ok",
            channel,
        ))),
            Err(e) => Err(e),
        }
}


#[get("/{channel_id}/")]
async fn read( data: Data<AppState>, path: Path<Info>,) -> impl Responder{
    info!("read");
    match Channel::read(&data.pool, path.channel_id).await{
        Ok(channel) => Ok(Json(channel)),
        Err(e) => Err(e),
    }
}
#[delete("/")]
async fn delete( data: Data<AppState>, path: Query<Info>,) -> impl Responder{
    info!("delete");
    match Channel::delete(&data.pool, path.channel_id).await{
            Ok(channel) => {
                info!("Remove directory {}/{}", FOLDER, &channel.id);
                match tokio::fs::remove_dir(format!("{}/{}", FOLDER, &channel.id))
                    .await {
                    Ok(_) => debug!("Removed directorio {}/{}", FOLDER, &channel.id),
                    Err(e) => error!("Can't remove directory {}/{}: {}", FOLDER, &channel.id, e),
                };
                Ok(Json(CustomResponse::new(
                    StatusCode::OK,
                    "Ok",
                    channel,
                )))
        },
        Err(e) => Err(e),
    }
}


async fn read_config_channels(
    data: Data<AppState>,
    page: Query<Page>,
) -> impl Responder{
    info!("read_all");
    let config = &data.config;
    let title = &config.title;
    let per_page = config.per_page;
    let page = page.page.unwrap_or(1);
    match Channel::read_with_pagination(&data.pool, page, per_page).await{
        Ok(channels) => {
            debug!("{:?}", channels);
            let template = ENV.get_template("config/channels.html").unwrap();
            let ctx = context! {
                app_title => title,
                channels => channels,

            };
            HttpResponse::Ok().body(template.render(ctx).unwrap())
        },
        Err(error) => {
            let template = ENV.get_template("error.html").unwrap();
            let ctx = context! {
                app_title => &title,
                error => error,
            };
            HttpResponse::Ok().body(template.render(ctx).unwrap())
        },
    }
}

async fn read_web_channels(
    data: Data<AppState>,
    page: Query<Page>,
) -> impl Responder{
    info!("read_web_channels");
    let config = &data.config;
    let title = &config.title;
    let per_page = config.per_page;
    let page = page.page.unwrap_or(1);
    let total = Channel::number_of_channels(&data.pool).await;
    match Channel::read_with_pagination(&data.pool, page, per_page).await{
        Ok(channels) => {
            debug!("{:?}", channels);
            let template = ENV.get_template("web/channels.html").unwrap();
            let ctx = context! {
                app_title => title,
                channels => channels,
                page => page,
                total => total / per_page + 1,
            };
            HttpResponse::Ok().body(template.render(ctx).unwrap())
        },
        Err(error) => {
            let template = ENV.get_template("error.html").unwrap();
            let ctx = context! {
                app_title => title,
                error => error,
            };
            HttpResponse::Ok().body(template.render(ctx).unwrap())
        },
    }
}
