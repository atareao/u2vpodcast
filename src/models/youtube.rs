use std::{collections::HashMap, fmt::Display};

use chrono::{DateTime, Duration};
use reqwest::{Client, StatusCode};
use serde_json::Value;
use std::fmt;
use async_recursion::async_recursion;


const URL: &'static str = "https://www.googleapis.com/youtube/v3";
const YTURL: &'static str = "https://www.youtube.com";
/*

                video = {"title": item['snippet']['title'],
                         "description": item['snippet']['description'],
                         "yt_id": video_id,
                         "link": link,
                         "published_at": item['snippet']['publishedAt']}
*/

#[derive(Debug)]
pub struct Video {
    title: String,
    description: String,
    yt_id: String,
    link: String,
    published_at: String,
    image: String,
    channel: String,
}

impl Video {
    pub fn new(title: &str, description: &str, yt_id: &str, link: &str,
            published_at: &str, image: &str, channel: &str) -> Self{
        Video{
            title: title.to_string(),
            description: description.to_string(),
            yt_id: yt_id.to_string(),
            link: link.to_string(),
            published_at: published_at.to_string(),
            image: image.to_string(),
            channel: channel.to_string(),
        }
    }
}
impl Display for Video {
    // add code here
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Title: {}\nDescription: {}\nId: {}\nLink: {}\nPublished At: {}\nImage: {}\nChannel: {}",
            self.title, self.description, self.yt_id, self.link,
            self.published_at, self.image, self.channel
        )
    }
}

#[derive(Debug)]
pub struct YouTube {
    key: String
}

impl YouTube {
    pub fn new(key: &str) -> Self{
        YouTube { key: key.to_string() }
    }

    #[async_recursion]
    pub async fn get_videos(&self, channel_id: String, after: Option<String>,
            next_token: Option<String>) ->Vec<Video>{
        println!("{} - {}", &self.key, &channel_id);
        let mut result: Vec<Video> = Vec::new();
        let url = format!("{}/search", URL);
        println!("{}", url);
        let mut params = HashMap::new();
        params.insert("part", "snippet");
        params.insert("channelId", &channel_id);
        params.insert("maxResults", "50");
        params.insert("order", "date");
        params.insert("type", "video");
        params.insert("key", &self.key);
        let published_at = match after.clone(){
            Some(value) => {
                let dt = DateTime::parse_from_rfc3339(&value).unwrap() + Duration::seconds(1);
                dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
            },
            None => "1970-01-01T00:00:00Z".to_string(),
        };
        params.insert("publishedAfter", &published_at);
        if next_token.is_some(){
            params.insert("pageToken", &next_token.as_ref().unwrap());
        }
        let client = Client::new();
        let response = client.get(url)
            .query(&params)
            .send()
            .await
            .unwrap();
        if response.status() == StatusCode::OK{
            let content = response.text()
                .await
                .unwrap();
            let data: Value = serde_json::from_str(&content).unwrap();
            for item in data.get("items").unwrap().as_array().unwrap(){
                let yt_id = item.get("id").unwrap().get("videoId").unwrap().as_str().unwrap();
                println!("{:?}", item);
                match item.get("snippet"){
                    Some(snippet) => {
                        let title = match snippet.get("title"){
                            Some(title) => title.as_str().unwrap(),
                            None => "",
                        };
                        let description = match snippet.get("description"){
                            Some(description) => description.as_str().unwrap(),
                            None => "",
                        };
                        let published_at = match snippet.get("publishedAt"){
                            Some(published_at) => published_at.as_str().unwrap(),
                            None => "",
                        };
                        let channel = match snippet.get("channelTitle"){
                            Some(channel) => channel.as_str().unwrap(),
                            None => "",
                        };
                        let image = match snippet.get("thumbnails"){
                            Some(item) => match item.get("high"){
                                Some(item) => match item.get("url"){
                                    Some(item) => item.as_str().unwrap(),
                                    None => "",
                                },
                                None => "",
                            },
                            None => "",
                        };
                        let link = format!("{}/watch?v={}", YTURL, yt_id);
                        let video = Video::new(title, description, yt_id,
                            &link, published_at, image, channel);
                        result.push(video);
                    },
                    None => {},
                }
            }
            if data.get("nextPageToken").is_some(){
                let next_token = Some(data.get("nextPageToken")
                    .unwrap().as_str().unwrap().to_string());
                let mut more_videos = self.get_videos(channel_id, after, next_token).await;
                result.append(&mut more_videos);
            }
        }else{
            println!("{:?}", response);
            println!("{}", response.status());
        }
        result
    }
}

#[cfg(test)]
mod tests{
    use dotenv::dotenv;

    use super::YouTube;

    #[actix_rt::test]
    async fn test_get_videos(){
        dotenv().ok();
        let key = std::env::var("YT_KEY").unwrap();
        let channel_id = std::env::var("YT_CHANNEL").unwrap();
        let after = Some("2022-10-19T20:20:03Z".to_string());
        let yt = YouTube::new(&key);
        let videos = yt.get_videos(channel_id, after, None).await;
        for video in &videos{
            println!("{}", video);
        }
        assert_ne!(videos.len(), 0)
    }
}
