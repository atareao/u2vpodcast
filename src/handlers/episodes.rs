use serde::Deserialize;
use actix_web::{
    Responder,
    HttpResponse,
    get,
    web::{
        self,
        ServiceConfig,
        Path,
        Data,
        Query, Json,
    },
};
use tracing::{
    info,
    debug,
};
use minijinja::context;

use crate::models::Channel;

use super::{
    ENV,
    AppState,
    super::models::Episode,
};


pub fn web_episodes(cfg: &mut ServiceConfig){
    cfg.service(
        web::resource("/channels/{channel_id}/episodes/")
            .route(web::get().to(read_web_episodes))
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

#[get("/{channel_id}/episodes/")]
async fn read_api_episodes(
    data: Data<AppState>,
    params: Query<Page>,
    path: Path<Info>
) -> impl Responder{
    info!("read_api_channels");
    let config = &data.config;
    let per_page = config.per_page;
    let channel_id = path.channel_id;
    let page = params.page.unwrap_or(1);
    match Episode::read_with_pagination(&data.pool, channel_id, page, per_page).await{
        Ok(episodes) => Ok(Json(episodes)),
        Err(e) => Err(e),
    }
}

async fn read_web_episodes(
    data: Data<AppState>,
    params: Query<Page>,
    path: Path<Info>
) -> impl Responder{
    info!("read_web_channels");
    let config = &data.config;
    let title = &config.title;
    let per_page = config.per_page;
    let channel_id = path.channel_id;
    let page = params.page.unwrap_or(1);
    let channel = Channel::read(&data.pool, channel_id)
        .await
        .map_err(|e| {
            let template = ENV.get_template("error.html").unwrap();
            let ctx = context! {
                app_title => &title,
                error => e,
            };
            HttpResponse::Ok().body(template.render(ctx).unwrap())
    }).unwrap();
    let total = Channel::total(&data.pool, channel_id).await;
    match Episode::read_with_pagination(&data.pool, channel_id, page, per_page).await{
        Ok(episodes) => {
            debug!("{:?}", episodes);
            let template = ENV.get_template("web/episodes.html").unwrap();
            let ctx = context! {
                app_title => title,
                channel => channel,
                episodes => episodes,
                page => page,
                total => total / per_page + 1,
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
