use tokio::process::Command;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

pub struct Ytdlp{
    path: String,
    cookies: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YtVideo{
    pub id: String,
    pub title: String,
    pub description: String,
    pub thumbnail: String,
    pub upload_date: String,
}

impl Ytdlp {
    pub fn new(path: &str, cookies: &str) -> Self{
        Self{
            path: path.to_string(),
            cookies: cookies.to_string(),
        }
    }
    pub async fn get_latest(&self, channel: &str, days: i32) -> Result<Vec<YtVideo>, anyhow::Error>{
        let url = format!("https://www.youtube.com/c/{}", channel);
        let elapsed = format!("today-{}days", days);
        let mut args = vec!["--dateafter", &elapsed, "--dump-json",
            "--break-on-reject"];
        args.push(&url);
        let stdout = Command::new(&self.path)
            .args(&args)
            .output()
            .await
            .map_err(|e| anyhow::anyhow!("Error"))
            .unwrap()
            .stdout;
        let mut result = std::str::from_utf8(&stdout)
            .unwrap()
            .split('\n')
            .collect::<Vec<&str>>()
            .join(",");
        result.pop();
        let content = format!("[{}]", result);
        let ytvideos: Vec<YtVideo> = serde_json::from_str(&content).unwrap();
        Ok(ytvideos)
    }

    pub async fn download(&self, id: &str, output: &str) -> std::process::ExitStatus{
        let url = format!("https://www.youtube.com/watch?v={}", id);
        let mut args = vec!["-f", "ba", "-x", "--audio-format", "mp3", 
            "-o", output];
        if &self.cookies != ""{
            args.push("--cookies");
            args.push(&self.cookies);
        }
        args.push(&url);
        Command::new(&self.path)
            .args(&args)
            .spawn()
            .expect("ytdlp can not start")
            .wait()
            .await
            .expect("ytdlp failed to run")
    }
}

#[tokio::test]
async fn test_e(){
    let ytdlp = Ytdlp::new("yt-dlp", "cookies.txt");
    let salida = ytdlp.get_latest("error", 0).await;
    match salida{
        Ok(videos) => {
            assert!(videos.len() == 0);
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
#[tokio::test]
async fn test_0(){
    let ytdlp = Ytdlp::new("yt-dlp", "cookies.txt");
    let salida = ytdlp.get_latest("atareao", 0).await;
    match salida{
        Ok(videos) => {
            assert!(videos.len() == 0);
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
#[tokio::test]
async fn test_info(){
    let ytdlp = Ytdlp::new("yt-dlp", "cookies.txt");
    let salida = ytdlp.get_latest("atareao", 5).await;
    match salida{
        Ok(videos) => {
            println!("{:?}", videos.get(0).unwrap());
            assert!(videos.len() > 0);
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}

async fn test_ytdlp(){
    let ytdlp = Ytdlp::new("yt-dlp", "cookies.txt");
    let salida = ytdlp.download("mWoJw5qD0eI", "/tmp/test.mp3").await;
    println!("{:?}", salida);
    assert!(true);
}
