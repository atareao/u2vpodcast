use actix_web::web;
use sqlx::{sqlite::SqlitePool, Error};
use super::episode::Episode;
use rss::{ChannelBuilder, ItemBuilder, extension::itunes::ITunesItemExtensionBuilder, EnclosureBuilder, GuidBuilder};

pub struct RSS{
    title: String,
    link: String,
    description: String,
}

impl RSS{
    pub fn new(title: &str, link: &str, description: &str) -> Self{
        Self{
            title: title.to_string(),
            link: link.to_string(),
            description: description.to_string(),
        }
    }

    pub async fn get_feed(&self, pool: &web::Data<SqlitePool>) -> Result<String, Error>{
        let episodes = Episode::read_all(pool).await.unwrap();
        let mut items = Vec::new();
        for episode in episodes{
            let itunes = ITunesItemExtensionBuilder::default()
                .image(Some(episode.image))
                .build();
            let enclosure = EnclosureBuilder::default()
                .url(episode.link)
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
        let channel = ChannelBuilder::default()
            .title(&self.title)
            .link(&self.link)
            .description(&self.description)
            .items(items)
            .build();

        Ok(channel.to_string())
    }
}
