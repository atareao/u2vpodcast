use axum::{
    http::{self, StatusCode},
    Router,
    Extension,
    extract::Path,
    routing::get, response::{IntoResponse, Response},
};
use serde::Deserialize;
use crate::models::{episode::Episode, channel::Channel};
use rss::{ChannelBuilder, ItemBuilder,
    extension::itunes::ITunesItemExtensionBuilder, 
    EnclosureBuilder, GuidBuilder};
use super::{ApiContext, error::{self, YTPError}};

pub fn router() -> Router {
    Router::new()
        .route("/rss/:path/feed.xml",
            get(feed)
        )
}
#[derive(Deserialize)]
pub struct RSS{
    title: String,
    description: String,
    url: String,
}

async fn feed(
    ctx: Extension<ApiContext>,
    Path(path): Path<String>,
) -> impl IntoResponse{
    tracing::info!("path: {}", path);
    if let Some(channel) = ctx.config.get_channel(&path){
        let episodes = Episode::read_all_in_channel(&ctx.pool, &channel.get_id()).await.unwrap();
        let mut items = Vec::new();
        for episode in episodes{
            let enclosure = format!("{}/media/{}/{}.mp3", ctx.config.get_url(), &path, episode.yt_id);
            let itunes = ITunesItemExtensionBuilder::default()
                .image(Some(episode.image))
                .build();
            let enclosure = EnclosureBuilder::default()
                .url(&enclosure)
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
        let link = format!("{}/rss", ctx.config.get_url());
        let channel_builder = ChannelBuilder::default()
            .title(channel.get_title().to_string())
            .description(channel.get_description().to_string())
            .link(&link)
            .items(items)
            .build();
        //Ok(channel_builder.to_string())
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-type", "application/rss+xml; charset=utf-8")
            .body(channel_builder.to_string())
            .unwrap()
    }else{
        YTPError::NotFound.get_response()
    }
}

