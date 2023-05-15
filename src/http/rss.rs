use std::sync::Arc;
use axum::{
    http::StatusCode,
    Router,
    extract::{Path, State},
    routing::get, response::{IntoResponse, Response},
};
use crate::models::{episode::Episode, channel::Channel};
use rss::{ChannelBuilder, ItemBuilder,
    extension::itunes::{ITunesItemExtensionBuilder, ITunesChannelExtensionBuilder}, 
    EnclosureBuilder, GuidBuilder};
use super::{AppState, error::YTPError};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/:path/feed.xml",
            get(feed)
        )
}

async fn feed(
    State(app_state): State<Arc<AppState>>,
    Path(channel_id): Path<i64>,
) -> impl IntoResponse{
    tracing::info!("path: {}", channel_id);
    if let Ok(option_channel) = Channel::read(&app_state.pool, channel_id).await{
        if let Some(channel) = option_channel{
            let episodes = Episode::read_all_in_channel(&app_state.pool, channel.get_id()).await.unwrap();
            let mut items = Vec::new();
            for episode in episodes{
                let enclosure = format!("{}/media/{}/{}.mp3", app_state.config.get_url(), &channel_id, episode.yt_id);
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
            let link = format!("{}/rss", app_state.config.get_url());
            let itunes = ITunesChannelExtensionBuilder::default()
                .image(channel.get_image())
                .summary(Some(channel.get_description().to_string()))
                .build();
            let channel_builder = ChannelBuilder::default()
                .title(channel.get_title().to_string())
                .description(channel.get_description().to_string())
                .link(&link)
                .itunes_ext(Some(itunes))
                .items(items)
                .build();
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-type", "application/rss+xml; charset=utf-8")
                .body(channel_builder.to_string())
                .unwrap()
        }else{
            YTPError::NotFound.get_response()
        }
    }else{
        YTPError::NotFound.get_response()
    }
}

