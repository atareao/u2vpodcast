use serde::Deserialize;
use sqlx::{sqlite::SqlitePool, Error};
use super::episode::Episode;
use super::rss::{ChannelBuilder, ItemBuilder, extension::itunes::ITunesItemExtensionBuilder, EnclosureBuilder, GuidBuilder};

#[derive(Deserialize)]
pub struct RSS{
    title: String,
    description: String,
    url: String,
}

impl RSS{
    pub fn new(title: &str, description: &str, url: &str) -> Self{
        Self{
            title: title.to_string(),
            description: description.to_string(),
            url: url.to_string(),
        }
    }

    pub async fn get_feed(&self, pool: &web::Data<SqlitePool>) -> Result<String, Error>{
        let episodes = Episode::read_all(pool).await.unwrap();
        let mut items = Vec::new();
        for episode in episodes{
            let enclosure = format!("{}/{}.mp3", self.url, episode.yt_id);
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
        let link = format!("{}/rss", self.url);
        let channel = ChannelBuilder::default()
            .title(&self.title)
            .description(&self.description)
            .link(&link)
            .items(items)
            .build();

        Ok(channel.to_string())
    }
}
