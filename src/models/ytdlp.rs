use tokio::process::Command;
use std::process::Command as StdCommand;
use serde::{Serialize, Deserialize};
use tracing::{
    info,
    debug,
};
use super::Error;

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
    pub original_url: String,
    pub webpage_url: String,
    pub upload_date: String,
    pub duration_string: String,
}

impl Ytdlp {
    pub fn new(path: &str, cookies: &str) -> Self{
        info!("new");
        Self{
            path: path.to_string(),
            cookies: cookies.to_string(),
        }
    }
    pub async fn get_latest(&self, url: &str, days: i64) -> Result<Vec<YtVideo>, Error>{
        info!("get_latest");
        let elapsed = format!("today-{}days", days);
        let args = vec!["--dateafter", &elapsed, "--dump-json",
            "--break-on-reject", url];
        let stdout = Command::new(&self.path)
            .args(&args)
            .output()
            .await?
            .stdout;
        let mut result = std::str::from_utf8(&stdout)?
            .split('\n')
            .collect::<Vec<&str>>()
            .join(",");
        result.pop();
        debug!("{}", &result);
        let content = format!("[{}]", result);
        let ytvideos: Vec<YtVideo> = serde_json::from_str(&content).unwrap();
        info!("{:?}", &ytvideos);
        Ok(ytvideos)
    }

    pub async fn download(&self, id: &str, output: &str) -> Result<std::process::ExitStatus, Error>{
        let url = format!("https://www.youtube.com/watch?v={}", id);
        let mut args = vec!["-f", "ba", "-x", "--audio-format", "mp3", 
            "-o", output];
        if !&self.cookies.is_empty(){
            args.push("--cookies");
            args.push(&self.cookies);
        }
        args.push(&url);
        Command::new(&self.path)
            .args(&args)
            .spawn()?
            .wait()
            .await
            .map_err(|e| e.into())
    }

    pub async fn auto_update() -> Result<(), Error>{
        let python3 = "python3";
        let args = vec!["-m", "pip", "install", "--user", "--upgrade",
            "--break-system-packages", "yt-dlp"];
        if StdCommand::new(python3)
            .args(&args)
            .spawn()
            .map_err(|e| Error::default(&e.to_string()))?
            .wait()
            .map_err(|e| Error::default(&e.to_string()))?
            .success(){
            Ok(())
        }else{
            Err(Error::default("Can't update yt-dlp"))
        }
    }
}

#[tokio::test]
async fn test_e(){
    let ytdlp = Ytdlp::new("yt-dlp", "cookies.txt");
    let salida = ytdlp.get_latest("error", 0).await;
    match salida{
        Ok(videos) => {
            assert!(videos.is_empty());
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
            assert!(videos.is_empty());
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
            println!("{:?}", videos.first().unwrap());
            assert!(!videos.is_empty());
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}

#[tokio::test]
async fn test_ytdlp(){
    let ytdlp = Ytdlp::new("yt-dlp", "cookies.txt");
    let salida = ytdlp.download("mWoJw5qD0eI", "/tmp/test.mp3").await;
    println!("{:?}", salida);
}

