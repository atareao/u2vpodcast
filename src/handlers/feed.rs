use serde::Deserialize;
use minijinja::context;
use actix_web::{
    HttpResponse,
    Responder,
    web::{
        self,
        ServiceConfig,
        Path,
        Data,
    },
};
use tracing::{info, error};
use rss::{
    ChannelBuilder,
    ItemBuilder,
    EnclosureBuilder,
    GuidBuilder,
    extension::itunes::{
        ITunesItemExtensionBuilder,
        ITunesChannelExtensionBuilder
    }, 
};

use crate::models::{Channel, Episode};

use super::{
    ENV,
    AppState,
};

#[derive(Deserialize)]
struct Info{
    channel_id: i64,
}

pub fn web_feed(cfg: &mut ServiceConfig){
    cfg.service(
        web::resource("/channels/{channel_id}/feed.xml")
            .route(web::get().to(get_feed))
    );
}

async fn get_feed(
    data: Data<AppState>,
    path: Path<Info>,
) -> impl Responder{
    info!("get_login");
    let config = &data.config;
    let url = &config.url;
    let app_title = &config.title;
    let channel_id = path.channel_id;
    match Channel::read(&data.pool, channel_id).await{
        Ok(channel) => {
            match Episode::read_all(&data.pool).await{
                Ok(episodes) => {
                    let mut items = Vec::new();
                    for episode in episodes{
                        let yt_id = &episode.yt_id;
                        let enclosure = format!("{url}/media/{channel_id}/{yt_id}.mp3");
                        let itunes = ITunesItemExtensionBuilder::default()
                            .image(Some(episode.image))
                            .summary(Some(episode.description.to_string()))
                            .explicit(Some("No".to_string()))
                            .episode_type(Some("Full".to_string()))
                            .duration(Some(episode.duration))
                            .build();
                        let enclosure = EnclosureBuilder::default()
                            .url(&enclosure)
                            .mime_type("audio/mpeg".to_string())
                            .build();
                        let guid = GuidBuilder::default()
                            .value(episode.yt_id)
                            .build();
                        let item = ItemBuilder::default()
                            .title(Some(episode.title))
                            .description(Some(episode.description))
                            .enclosure(Some(enclosure))
                            .guid(Some(guid))
                            .pub_date(Some(episode.published_at.to_string()))
                            .itunes_ext(Some(itunes))
                            .build();
                        items.push(item);
                    }
                    let link = format!("{url}/rss");
                    let itunes = ITunesChannelExtensionBuilder::default()
                        .image(Some(channel.image))
                        .summary(Some(channel.description.clone()))
                        .build();
                    let channel_builder = ChannelBuilder::default()
                        .title(channel.title)
                        .description(channel.description)
                        .link(&link)
                        .itunes_ext(Some(itunes))
                        .items(items)
                        .build();
                    HttpResponse::Ok()
                        .append_header(("Content-type", "application/rss+xml; charset=utf-8"))
                        .body(channel_builder.to_string())
                },
                Err(e) => {
                    error!("{e}");
                    let template = ENV.get_template("error.html").unwrap();
                    let ctx = context! {
                        app_title => app_title,
                        error => e,
                    };
                    HttpResponse::Ok().body(template.render(ctx).unwrap())
                },
            }
        },
        Err(e) => {
            error!("{e}");
            let template = ENV.get_template("error.html").unwrap();
            let ctx = context! {
                app_title => app_title,
                error => e,
            };
            HttpResponse::Ok().body(template.render(ctx).unwrap())
        },
    }
}
