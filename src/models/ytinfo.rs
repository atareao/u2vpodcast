use regex::Regex;
use ureq;

use super::Error;



#[derive(Debug, Clone)]
pub struct YTInfo{
    pub title: String,
    pub description: String,
    pub image: String,
}

impl YTInfo{
    pub fn default() -> Self {
        Self{
            title: "".to_string(),
            description: "".to_string(),
            image: "".to_string(),
        }
    }

    pub async fn new(url: &str) -> Result<Self, Error>{

        let html: String = ureq::get(url)
            .call()
            .map_err(|e| Error::new(&e.to_string()))?
            .into_string()
            .map_err(|e| Error::new(&e.to_string()))?;

        let title = get_metadata(&html, "og:title");
        let description = get_metadata(&html, "og:description");
        let image = get_image(&html);

        Ok(Self{
            title,
            description,
            image,
        })
    }
}

fn get_image(html: &str) -> String{
    let pattern = r#"meta\s+property="og:image"\s+content="(?P<content>[^"]*)""#;
    let re = Regex::new(pattern).unwrap();
    re.captures(html)
        .map(|c| {
            let part = c["content"].to_string();
            part.find('?')
            .map(|pos| part[..pos].to_string())
            .unwrap_or(part)
        })
        .unwrap_or("".to_string())
}

fn get_metadata(html: &str, metadata: &str) -> String{
    let pattern = format!(r#"meta\s+property="{}"\s+content="(?P<content>[^"]*)""#,
        metadata);
    let re = Regex::new(&pattern).unwrap();
    re.captures(html)
        .map(|c| c["content"].to_string())
        .unwrap_or("".to_string())
        
}



#[tokio::test]
async fn test_info_channel(){
    let url = "https://www.youtube.com/c/atareao";
    let ytinfo = YTInfo::new(url).await;
    println!("{:?}", ytinfo);
    assert!(ytinfo.is_ok())
}

#[tokio::test]
async fn test_info_playlist(){
    let url = "https://www.youtube.com/playlist?list=PL3lTiK2rXrUFdTzriDsmNCG28T8u7bhEd";
    let ytinfo = YTInfo::new(url).await;
    println!("{:?}", ytinfo);
    assert!(ytinfo.is_ok())
}

#[tokio::test]
async fn test_info_video(){
    let url = "https://www.youtube.com/watch?v=2A1abiQJAiM";
    let ytinfo = YTInfo::new(url).await;
    println!("{:?}", ytinfo);
    assert!(ytinfo.is_ok())
}
