use actix_web::{
    web::{self, Data, Path, ServiceConfig},
    HttpResponse, Responder,
};
use rss::{
    extension::itunes::{ITunesChannelExtensionBuilder, ITunesItemExtensionBuilder},
    extension::{Extension, ExtensionMap},
    ChannelBuilder, EnclosureBuilder, GuidBuilder, ItemBuilder,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::models::{audios_dir, Channel, Episode, SponsorBlockCache};
use crate::utils::middleware::SessionOrBasicAuth;
use crate::utils::sponsorblock::{
    parse_duration_seconds, rejected_intervals, retained_intervals, translate_chapters,
};
use std::{collections::BTreeMap, path::Path as FsPath};

use super::AppState;

#[derive(Deserialize)]
struct Info {
    key: String,
}

#[derive(Deserialize)]
struct ChaptersInfo {
    slug: String,
    yt_id: String,
}

#[derive(Serialize)]
struct PodcastChapter {
    #[serde(rename = "startTime")]
    start_time: f64,
    title: String,
}

#[derive(Serialize)]
struct PodcastChapters {
    version: &'static str,
    chapters: Vec<PodcastChapter>,
}

const PODCAST_NAMESPACE: &str = "https://podcastindex.org/namespace/1.0";
const PODCAST_CHAPTERS_VERSION: &str = "1.2.0";

pub fn web_feed(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/channels/{key}/feed.xml")
            .route(web::get().to(get_feed))
            .wrap(SessionOrBasicAuth),
    )
    .service(
        web::resource("/{key}/feed.xml")
            .route(web::get().to(get_legacy_feed))
            .wrap(SessionOrBasicAuth),
    )
    .service(
        web::resource("/feed.xml")
            .route(web::get().to(get_global_feed))
            .wrap(SessionOrBasicAuth),
    )
    .service(
        web::resource("/channels/{slug}/episodes/{yt_id}/chapters.json")
            .route(web::get().to(get_chapters))
            .wrap(SessionOrBasicAuth),
    );
}

async fn get_chapters(data: Data<AppState>, path: Path<ChaptersInfo>) -> HttpResponse {
    let path = path.into_inner();
    let episode = match Episode::read_by_yt_id_with_channel(&data.pool, &path.yt_id).await {
        Ok(episode) if episode.channel_slug == path.slug => episode,
        Ok(_) => return HttpResponse::NotFound().finish(),
        Err(error) => {
            error!("Error reading chapters episode: {error}");
            return HttpResponse::build(error.status_code()).finish();
        }
    };

    let channel_dir = FsPath::new(audios_dir()).join(&path.slug);
    let selected = episode.selected_media(&channel_dir, data.config.sponsorblock_enabled);
    let processed_selected =
        episode.sponsorblock_processed_filename.as_deref() == Some(selected.filename.as_str());
    let chapters = if processed_selected {
        let cache = match SponsorBlockCache::read(&data.pool, episode.id).await {
            Ok(Some(cache)) => cache,
            Ok(None) => return HttpResponse::InternalServerError().finish(),
            Err(error) => {
                error!("Error reading SponsorBlock chapters state: {error}");
                return HttpResponse::InternalServerError().finish();
            }
        };
        let Some(duration) = parse_duration_seconds(&episode.duration) else {
            return HttpResponse::InternalServerError().finish();
        };
        let rejected = rejected_intervals(
            &cache.segments,
            &data.config.sponsorblock_rejected_categories,
        );
        let retained = retained_intervals(&rejected, duration);
        translate_chapters(&episode.chapters, &retained)
    } else {
        episode.chapters
    };

    let chapters = chapters
        .into_iter()
        .map(|chapter| PodcastChapter {
            start_time: if processed_selected {
                (chapter.start * 1000.0).round() / 1000.0
            } else {
                chapter.start
            },
            title: chapter.title,
        })
        .collect();
    HttpResponse::Ok()
        .content_type("application/json+chapters")
        .json(PodcastChapters {
            version: PODCAST_CHAPTERS_VERSION,
            chapters,
        })
}

fn podcast_namespaces() -> BTreeMap<String, String> {
    BTreeMap::from([("podcast".to_string(), PODCAST_NAMESPACE.to_string())])
}

async fn get_global_feed(data: Data<AppState>) -> impl Responder {
    info!("get_global_feed");
    let url = &data.config.url;
    match Episode::read_all_with_channels(&data.pool).await {
        Ok(episodes) => {
            let items = global_items(
                url,
                FsPath::new(audios_dir()),
                episodes,
                data.config.sponsorblock_enabled,
            );
            let link = format!("{url}/rss");
            let itunes = ITunesChannelExtensionBuilder::default()
                .summary(Some(
                    "All episodes from all channels, newest first".to_string(),
                ))
                .build();
            let channel_builder = ChannelBuilder::default()
                .title("All Episodes")
                .description("All episodes from all channels, newest first")
                .link(&link)
                .itunes_ext(Some(itunes))
                .namespaces(podcast_namespaces())
                .items(items)
                .build();
            HttpResponse::Ok()
                .append_header(("Content-type", "application/rss+xml; charset=utf-8"))
                .body(channel_builder.to_string())
        }
        Err(e) => {
            error!("Error: {e}");
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

async fn get_feed(data: Data<AppState>, path: Path<Info>) -> impl Responder {
    info!("get_feed");
    let key = path.key.clone();
    match Channel::read_by_id_or_slug(&data.pool, &key).await {
        Ok(channel) => build_feed(&data, channel).await,
        Err(e) => {
            error!("Error: {e}");
            HttpResponse::NotFound().body("Feed not found")
        }
    }
}

async fn get_legacy_feed(data: Data<AppState>, path: Path<Info>) -> impl Responder {
    info!("get_legacy_feed");
    let key = path.key.clone();
    match Channel::read_by_slug(&data.pool, &key).await {
        Ok(channel) => build_feed(&data, channel).await,
        Err(e) => {
            error!("Error: {e}");
            HttpResponse::NotFound().body("Feed not found")
        }
    }
}

async fn build_feed(data: &Data<AppState>, channel: Channel) -> HttpResponse {
    let url = &data.config.url;
    match Episode::read_episodes_for_channel(&data.pool, channel.id).await {
        Ok(episodes) => {
            let items = channel_items(
                url,
                FsPath::new(audios_dir()),
                &channel.slug,
                episodes,
                data.config.sponsorblock_enabled,
            );
            let link = format!("{url}/rss");
            let itunes = ITunesChannelExtensionBuilder::default()
                .image(Some(channel.image))
                .summary(Some(channel.description.clone()))
                .build();
            let channel_builder = ChannelBuilder::default()
                .title(channel.title)
                .description(channel.description)
                .link(&link)
                .itunes_ext(Some(itunes))
                .namespaces(podcast_namespaces())
                .items(items)
                .build();
            HttpResponse::Ok()
                .append_header(("Content-type", "application/rss+xml; charset=utf-8"))
                .body(channel_builder.to_string())
        }
        Err(e) => {
            error!("Error: {e}");
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

fn episode_item(
    url: &str,
    audio_root: &FsPath,
    slug: &str,
    episode: Episode,
    title: String,
    sponsorblock_enabled: bool,
) -> rss::Item {
    let channel_dir = audio_root.join(slug);
    let selected = episode.selected_media(&channel_dir, sponsorblock_enabled);
    let enclosure_url = format!("{url}/media/{slug}/{}.mp3", episode.yt_id);
    let enclosure_length = std::fs::metadata(channel_dir.join(&selected.filename))
        .map(|metadata| metadata.len())
        .unwrap_or(0);
    let chapters_url = format!(
        "{url}/channels/{slug}/episodes/{}/chapters.json",
        episode.yt_id
    );
    let description = format!("{}\n\n{}", episode.webpage_url, episode.description);
    let itunes = ITunesItemExtensionBuilder::default()
        .image(Some(episode.image))
        .summary(Some(description.clone()))
        .explicit(Some("No".to_string()))
        .episode_type(Some("Full".to_string()))
        .duration(Some(selected.duration))
        .build();
    let enclosure = EnclosureBuilder::default()
        .url(enclosure_url)
        .length(enclosure_length.to_string())
        .mime_type("audio/mpeg".to_string())
        .build();
    let mut item = ItemBuilder::default()
        .title(Some(title))
        .description(Some(description))
        .enclosure(Some(enclosure))
        .guid(Some(
            GuidBuilder::default()
                .value(episode.yt_id.clone())
                .permalink(false)
                .build(),
        ))
        .pub_date(Some(episode.published_at.to_rfc2822()))
        .itunes_ext(Some(itunes))
        .build();
    if !episode.chapters.is_empty() {
        let extension = Extension {
            name: "podcast:chapters".to_string(),
            attrs: BTreeMap::from([
                ("url".to_string(), chapters_url),
                ("type".to_string(), "application/json+chapters".to_string()),
            ]),
            ..Extension::default()
        };
        let extensions: ExtensionMap = BTreeMap::from([(
            "podcast".to_string(),
            BTreeMap::from([("chapters".to_string(), vec![extension])]),
        )]);
        item.set_extensions(extensions);
    }
    item
}

fn channel_items(
    url: &str,
    audio_root: &FsPath,
    slug: &str,
    episodes: Vec<Episode>,
    sponsorblock_enabled: bool,
) -> Vec<rss::Item> {
    episodes
        .into_iter()
        .map(|episode| {
            let title = episode.title.clone();
            episode_item(url, audio_root, slug, episode, title, sponsorblock_enabled)
        })
        .collect()
}

fn global_items(
    url: &str,
    audio_root: &FsPath,
    episodes: Vec<Episode>,
    sponsorblock_enabled: bool,
) -> Vec<rss::Item> {
    episodes
        .into_iter()
        .filter(|episode| !episode.channel_slug.is_empty())
        .map(|episode| {
            let slug = episode.channel_slug.clone();
            let title = if episode.channel_title.is_empty() {
                episode.title.clone()
            } else {
                format!("{}: {}", episode.channel_title, episode.title)
            };
            episode_item(url, audio_root, &slug, episode, title, sponsorblock_enabled)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{config::test_config, SponsorBlockCache, SponsorBlockSegment};
    use crate::utils::sponsorblock::{generate_processed_mp3, processing_hash, snapshot_hash};
    use actix_web::{http::StatusCode, test, App};
    use chrono::{Duration, Utc};
    use sqlx::{migrate::Migrator, sqlite::SqlitePoolOptions};
    use std::process::Command;

    async fn fixture() -> (sqlx::SqlitePool, i64, std::path::PathBuf) {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        Migrator::new(FsPath::new(env!("CARGO_MANIFEST_DIR")).join("migrations"))
            .await
            .unwrap()
            .run(&pool)
            .await
            .unwrap();
        let now = Utc::now();
        let channel_id: i64 = sqlx::query_scalar(
            "INSERT INTO channels (url, title, slug, active, description, image, first, max, created_at, updated_at) \
             VALUES ('https://example.com', 'Channel title', 'channel_slug', TRUE, '', '', $1, 5, $1, $1) RETURNING id",
        )
        .bind(now)
        .fetch_one(&pool)
        .await
        .unwrap();
        for (index, yt_id) in ["processed", "empty", "missing"].iter().enumerate() {
            sqlx::query(
                "INSERT INTO episodes (channel_id, title, yt_id, webpage_url, published_at, duration, created_at, updated_at) \
                 VALUES ($1, $2, $3, $4, $5, '600', $5, $5)",
            )
            .bind(channel_id)
            .bind(format!("Episode {yt_id}"))
            .bind(yt_id)
            .bind(format!("https://example.com/{yt_id}"))
            .bind(now - Duration::seconds(index as i64))
            .execute(&pool)
            .await
            .unwrap();
        }
        sqlx::query("UPDATE episodes SET chapters_json = $1 WHERE yt_id IN ('processed', 'empty')")
            .bind(r#"[{"start":0.0,"end":600.0,"title":"Whole episode"}]"#)
            .execute(&pool)
            .await
            .unwrap();
        let root = std::env::temp_dir().join(format!("u2vpodcast-feed-{}", rand::random::<u64>()));
        std::fs::create_dir_all(root.join("channel_slug")).unwrap();
        (pool, channel_id, root)
    }

    #[actix_web::test]
    async fn simulated_feeds_select_processed_media_and_fallbacks() {
        let (pool, channel_id, root) = fixture().await;
        let episodes = Episode::read_episodes_for_channel(&pool, channel_id)
            .await
            .unwrap();
        let by_id = |yt_id: &str| {
            episodes
                .iter()
                .find(|episode| episode.yt_id == yt_id)
                .unwrap()
                .id
        };
        SponsorBlockCache::upsert_success(
            &pool,
            by_id("processed"),
            &[],
            "processed-hash",
            "processed-hash",
            Some("processed.sponsorblock.abcdef.mp3"),
            Some(540.0),
        )
        .await
        .unwrap();
        SponsorBlockCache::upsert_success(
            &pool,
            by_id("empty"),
            &[],
            "empty-hash",
            "empty-hash",
            None,
            None,
        )
        .await
        .unwrap();
        SponsorBlockCache::upsert_success(
            &pool,
            by_id("missing"),
            &[],
            "missing-hash",
            "missing-hash",
            Some("missing.sponsorblock.abcdef.mp3"),
            Some(500.0),
        )
        .await
        .unwrap();
        std::fs::write(
            root.join("channel_slug/processed.sponsorblock.abcdef.mp3"),
            b"fixture",
        )
        .unwrap();

        let channel_episodes = Episode::read_episodes_for_channel(&pool, channel_id)
            .await
            .unwrap();
        let items = channel_items(
            "http://backend",
            &root,
            "channel_slug",
            channel_episodes,
            true,
        );
        let processed = items
            .iter()
            .find(|item| item.guid().unwrap().value() == "processed")
            .unwrap();
        assert!(processed
            .enclosure()
            .unwrap()
            .url()
            .ends_with("/processed.mp3"));
        assert!(!processed
            .enclosure()
            .unwrap()
            .url()
            .contains(".sponsorblock."));
        assert_eq!(processed.enclosure().unwrap().length(), "7");
        assert_eq!(processed.itunes_ext().unwrap().duration(), Some("540"));
        for yt_id in ["empty", "missing"] {
            let item = items
                .iter()
                .find(|item| item.guid().unwrap().value() == yt_id)
                .unwrap();
            assert!(item
                .enclosure()
                .unwrap()
                .url()
                .ends_with(&format!("/{yt_id}.mp3")));
            assert_eq!(item.enclosure().unwrap().length(), "0");
            assert_eq!(item.itunes_ext().unwrap().duration(), Some("600"));
        }

        let global = global_items(
            "http://backend",
            &root,
            Episode::read_all_with_channels(&pool).await.unwrap(),
            true,
        );
        assert_eq!(global.len(), 3);
        assert!(global
            .iter()
            .all(|item| item.title().unwrap().starts_with("Channel title: ")));
        assert_eq!(processed.guid().unwrap().value(), "processed");

        let disabled_channel = channel_items(
            "http://backend",
            &root,
            "channel_slug",
            Episode::read_episodes_for_channel(&pool, channel_id)
                .await
                .unwrap(),
            false,
        );
        let disabled = disabled_channel
            .iter()
            .find(|item| item.guid().unwrap().value() == "processed")
            .unwrap();
        assert!(disabled
            .enclosure()
            .unwrap()
            .url()
            .ends_with("/processed.mp3"));
        assert_eq!(disabled.itunes_ext().unwrap().duration(), Some("600"));

        let disabled_global = global_items(
            "http://backend",
            &root,
            Episode::read_all_with_channels(&pool).await.unwrap(),
            false,
        );
        let disabled = disabled_global
            .iter()
            .find(|item| item.guid().unwrap().value() == "processed")
            .unwrap();
        assert!(disabled
            .enclosure()
            .unwrap()
            .url()
            .ends_with("/processed.mp3"));
        assert_eq!(disabled.itunes_ext().unwrap().duration(), Some("600"));

        for yt_id in ["processed", "empty"] {
            let item = items
                .iter()
                .find(|item| item.guid().unwrap().value() == yt_id)
                .unwrap();
            let chapters = &item.extensions()["podcast"]["chapters"][0];
            assert_eq!(chapters.name(), "podcast:chapters");
            assert_eq!(
                chapters.attrs()["url"],
                format!("http://backend/channels/channel_slug/episodes/{yt_id}/chapters.json")
            );
            assert_eq!(chapters.attrs()["type"], "application/json+chapters");
        }
        let missing = items
            .iter()
            .find(|item| item.guid().unwrap().value() == "missing")
            .unwrap();
        assert!(missing.extensions().is_empty());

        let xml = ChannelBuilder::default()
            .title("Channel")
            .description("Description")
            .link("https://example.com")
            .namespaces(podcast_namespaces())
            .items(items)
            .build()
            .to_string();
        assert_eq!(xml.matches("xmlns:podcast=").count(), 1);
        assert_eq!(xml.matches("<podcast:chapters ").count(), 2);
        let parsed = rss::Channel::read_from(xml.as_bytes()).unwrap();
        assert_eq!(parsed.namespaces()["podcast"], PODCAST_NAMESPACE);
        // The guid is the episode's stable public id, not a URL: it must be
        // declared as a non-permalink so podcatchers key episodes by that id
        // instead of falling back to the (previously changing) enclosure URL.
        assert!(xml.contains(r#"<guid isPermaLink="false">"#));
        for item in parsed.items() {
            assert!(
                !item.guid().unwrap().is_permalink(),
                "guid must not be a permalink"
            );
        }
        std::fs::remove_dir_all(root).unwrap();
    }

    fn probed_chapters(path: &FsPath) -> serde_json::Value {
        let output = Command::new("ffprobe")
            .args(["-v", "error", "-show_chapters", "-of", "json"])
            .arg(path)
            .output()
            .expect("start ffprobe");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let probe: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        let chapters = probe["chapters"]
            .as_array()
            .unwrap()
            .iter()
            .map(|chapter| {
                serde_json::json!({
                    "startTime": chapter["start_time"].as_str().unwrap().parse::<f64>().unwrap(),
                    "title": chapter["tags"]["title"],
                })
            })
            .collect::<Vec<_>>();
        serde_json::json!({ "version": PODCAST_CHAPTERS_VERSION, "chapters": chapters })
    }

    #[actix_web::test]
    async fn chapters_endpoint_matches_original_processed_and_empty_timelines() {
        let (pool, channel_id, fixture_root) = fixture().await;
        let slug = format!("chapters_endpoint_{}", rand::random::<u64>());
        sqlx::query("UPDATE channels SET slug = $1 WHERE id = $2")
            .bind(&slug)
            .bind(channel_id)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("UPDATE episodes SET duration = '3', chapters_json = $1 WHERE yt_id = 'processed'")
            .bind(
                r#"[{"start":0.1234,"end":1.0,"title":"Intro"},{"start":1.2,"end":1.8,"title":"Removed"},{"start":0.5555,"end":3.0,"title":"Main"}]"#,
            )
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("UPDATE episodes SET duration = '3', chapters_json = $1 WHERE yt_id = 'empty'")
            .bind(r#"[{"start":0.0,"end":3.0,"title":"Untrimmed"}]"#)
            .execute(&pool)
            .await
            .unwrap();

        let directory = FsPath::new(audios_dir()).join(&slug);
        std::fs::create_dir_all(&directory).unwrap();
        let original = directory.join("processed.mp3");
        let output = Command::new("ffmpeg")
            .args([
                "-hide_banner",
                "-loglevel",
                "error",
                "-f",
                "lavfi",
                "-i",
                "sine=frequency=440:duration=3",
                "-q:a",
                "4",
                "-y",
            ])
            .arg(&original)
            .output()
            .expect("start fixture ffmpeg");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );

        let segments = [SponsorBlockSegment::new(1.0, 2.0, "sponsor")];
        let categories = ["sponsor".to_string()];
        let hash = processing_hash(&segments, &categories);
        let processed_episode = Episode::read_by_yt_id_with_channel(&pool, "processed")
            .await
            .unwrap();
        let processed = generate_processed_mp3(
            &original,
            &segments,
            3.0,
            &hash,
            &processed_episode.chapters,
        )
        .await
        .unwrap();
        SponsorBlockCache::upsert_success(
            &pool,
            processed_episode.id,
            &segments,
            &snapshot_hash(&segments),
            &hash,
            Some(&processed.filename),
            Some(processed.duration),
        )
        .await
        .unwrap();

        let empty_episode = Episode::read_by_yt_id_with_channel(&pool, "empty")
            .await
            .unwrap();
        let empty_segments = [];
        SponsorBlockCache::upsert_success(
            &pool,
            empty_episode.id,
            &empty_segments,
            &snapshot_hash(&empty_segments),
            &processing_hash(&empty_segments, &categories),
            None,
            None,
        )
        .await
        .unwrap();

        let mut config = test_config();
        config.sponsorblock_enabled = true;
        config.sponsorblock_rejected_categories = categories.to_vec();
        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState {
                    config,
                    pool: pool.clone(),
                }))
                .service(
                    web::resource("/channels/{slug}/episodes/{yt_id}/chapters.json")
                        .route(web::get().to(get_chapters)),
                ),
        )
        .await;

        let processed_response = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&format!(
                    "/channels/{slug}/episodes/processed/chapters.json"
                ))
                .to_request(),
        )
        .await;
        assert_eq!(processed_response.status(), StatusCode::OK);
        assert_eq!(
            processed_response.headers().get("content-type").unwrap(),
            "application/json+chapters"
        );
        let body: serde_json::Value = test::read_body_json(processed_response).await;
        assert_eq!(body, probed_chapters(&directory.join(&processed.filename)));

        // SponsorBlock is enabled, but an authoritative empty snapshot keeps
        // the original enclosure and therefore the original chapter times.
        let empty_response = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&format!("/channels/{slug}/episodes/empty/chapters.json"))
                .to_request(),
        )
        .await;
        assert_eq!(empty_response.status(), StatusCode::OK);
        let body: serde_json::Value = test::read_body_json(empty_response).await;
        assert_eq!(
            body,
            serde_json::json!({
                "version": PODCAST_CHAPTERS_VERSION,
                "chapters": [{"startTime": 0.0, "title": "Untrimmed"}]
            })
        );

        let missing_response = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(&format!("/channels/{slug}/episodes/missing/chapters.json"))
                .to_request(),
        )
        .await;
        assert_eq!(missing_response.status(), StatusCode::OK);
        let body: serde_json::Value = test::read_body_json(missing_response).await;
        assert_eq!(
            body,
            serde_json::json!({"version": PODCAST_CHAPTERS_VERSION, "chapters": []})
        );

        std::fs::remove_dir_all(directory).unwrap();
        std::fs::remove_dir_all(fixture_root).unwrap();
    }
}
