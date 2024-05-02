use sqlx::SqlitePool;
use tracing::{
    info,
    error,
};
use chrono::{
    Utc,
    DateTime,
    TimeZone,
    naive::{
        NaiveDate,
        NaiveDateTime
    },
};
use std::convert::TryFrom;
use tokio::fs::create_dir_all;
use super::super::models::{
    Error,
    Channel,
    Episode,
    Ytdlp,
    YtVideo,
};

static FOLDER: &str = "/app/audios";
static YTDLP: &str = "/app/.local/bin/yt-dlp";

pub async fn do_the_work(pool: &SqlitePool) -> Result<(), Error>{
    let ytdlp = Ytdlp::new(YTDLP, "cookies-cp.txt");
    let channels = Channel::read_all(pool).await?;
    for channel in channels.as_slice(){
        info!("Processing: {}", channel.url);
        match process_channel(pool, channel, &ytdlp).await{
            Ok(_) => {},
            Err(e) => error!{"Cant process channel: {channel}. Error: {e}"},
        }
        match clean_channel(pool, channel).await{
            Ok(()) => info!("Channel {} cleaned", &channel.id),
            Err(e) => error!("Can't clean channel {}. {}", &channel.id, e),
        }
    }
    Ok(())
}

async fn clean_channel(pool: &SqlitePool, channel: &Channel) -> Result<(), Error>{
    let max = usize::try_from(channel.max)
        .map_err(|e| Error::default(&e.to_string()))?;
    let episodes = Episode::read_episodes_for_channel(pool, channel.id).await?;
    for (index, episode) in episodes.iter().enumerate(){
        if index >= max { // remove
            let filename = format!("{}/{}/{}.mp3", FOLDER, &channel.id, episode.yt_id);
            info!("Deleting file {filename}");
            let exists = tokio::fs::metadata(&filename)
                .await
                .map(|f| f.is_file())
                .unwrap_or(false);
            let removed = tokio::fs::remove_file(&filename)
                .await
                .map(|_| true)
                .unwrap_or(false);
            if !exists || removed {
                match Episode::remove(pool, episode.id).await{
                    Ok(_) => info!("Removed {}", &filename),
                    Err(e) => error!("Cant remove {}. {}", &filename, e),
                }
            }
        }
    }
    Ok(())
}

async fn process_channel(
    pool: &SqlitePool,
    channel: &Channel,
    ytdlp: &Ytdlp,
) -> Result<(), Error>{
    info!("Create directory {}/{}", FOLDER, &channel.id);
    let _ = create_dir_all(format!("{}/{}", FOLDER, &channel.id))
        .await;
    info!("Getting new videos for channel: {}", channel);
    let first = channel.first;
    let last = if channel.number_of_episodes(pool).await > 0 {
        let last = channel.get_max_date(pool).await;
        if last < first{
            first
        }else{
            last
        }
    }else{
        first
    };
    info!("Last video: {}", &last);
    let days = (Utc::now().timestamp() - last.timestamp())/86400;
    info!("Number of days: {}", days);
    let ytvideos = ytdlp.get_latest(&channel.url, days).await?;
    info!("Getting {} videos", ytvideos.len());
    for ytvideo in ytvideos{
        info!("Processing: {}", &ytvideo.title);
        match process_episode(pool, channel, &ytvideo, ytdlp).await{
            Ok(_) => {},
            Err(e) => error!("Cant process episode: {e}"),
        }
    }
    //TODO: Delete older episodes
    Ok(())
}

async fn process_episode(
    pool: &SqlitePool,
    channel: &Channel,
    ytvideo: &YtVideo,
    ytdlp: &Ytdlp,
) -> Result<(), Error>{
    info!("Start processing episode {}", ytvideo.title);
    if channel.episode_exists(pool, &ytvideo.id).await{
        info!("El video {} titulado '{}', existe",
            &ytvideo.id,
            &ytvideo.title
        );
        return Ok(());
    }
    info!("Downloading video: {:?}", ytvideo);
    let filename = format!("{}/{}/{}.mp3",
        FOLDER,
        channel.id,
        &ytvideo.id
    );

    if !ytdlp.download(&ytvideo.id, &filename).await?.success(){
        Err(Error::default(&format!("Cant download {filename}")))?
    }
    let title = &ytvideo.title;
    let description = &ytvideo.description;
    let yt_id = &ytvideo.id;
    let webpage_url = &ytvideo.webpage_url;
    let duration = &ytvideo.duration_string;
    info!("{}", &ytvideo.upload_date);
    let published_at = parse_date(&ytvideo.upload_date);
    let _ = filetime::set_file_mtime(
        &filename,
        filetime::FileTime::from_unix_time(
            published_at.timestamp(), 0)
    );
    let image = &ytvideo.thumbnail;
    let listen = false;
    let _ = Episode::new(
        pool,
        channel.id,
        title,
        description,
        yt_id,
        webpage_url,
        &published_at,
        duration,
        image,
        listen
    ).await?;
    Ok(())
}

fn parse_date(date: &str) -> DateTime<Utc>{
    let format = "%Y%m%d";
    let naive_date = NaiveDate::parse_from_str(date, format).unwrap();
    // Add some default time to convert it into a NaiveDateTime
    let naive_datetime: NaiveDateTime = naive_date.and_hms_opt(0,0,0).unwrap();
    // Add a timezone to the object to convert it into a DateTime<UTC>
    TimeZone::from_utc_datetime(&Utc, &naive_datetime)
}

