use axum::{
    Router,
    Extension,
    extract::Path,
    routing::get,
};
use serde::Deserialize;
use crate::models::{episode::Episode, channel::Channel};
use rss::{ChannelBuilder, ItemBuilder,
    extension::itunes::ITunesItemExtensionBuilder, 
    EnclosureBuilder, GuidBuilder};
use super::{ApiContext, error};

pub fn router() -> Router {
    Router::new()
        .route("/rss/:path",
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
) -> Result<String, error::Error>{
        let channel = Channel::read_by_path(&ctx.pool, &path).await?;
        let episodes = Episode::read_all(&ctx.pool).await.unwrap();
        let mut items = Vec::new();
        for episode in episodes{
            let enclosure = format!("{}/{}.mp3", ctx.config.get_url(), episode.yt_id);
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
            .title(&channel.title)
            .description(&channel.description)
            .link(&link)
            .items(items)
            .build();
        Ok(channel_builder.to_string())
    }

