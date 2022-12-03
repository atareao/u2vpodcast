use std::io::Read;

use tokio::process::Command;

pub struct Ytdlp{
    path: String,
    cookies: String,
}

impl Ytdlp {
    pub fn new(path: &str, cookies: &str) -> Self{
        Self{
            path: path.to_string(),
            cookies: cookies.to_string(),
        }
    }
    pub async fn get_latest(&self, channel: &str, days: i32) -> Result<String, anyhow::Error>{
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
        Ok(std::str::from_utf8(&stdout).unwrap().to_string())
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
async fn test_info(){
    let ytdlp = Ytdlp::new("yt-dlp", "cookies.txt");
    let salida = ytdlp.get_latest("atareao", 2).await;
    println!("{:?}", salida);
    assert!(true);
}

async fn test_ytdlp(){
    let ytdlp = Ytdlp::new("yt-dlp", "cookies.txt");
    let salida = ytdlp.download("mWoJw5qD0eI", "/tmp/test.mp3").await;
    println!("{:?}", salida);
    assert!(true);
}
