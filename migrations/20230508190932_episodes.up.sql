-- pub struct Episode {
--     id: i64,
--     channel_id: i64,
--     title: String,
--     description: String,
--     yt_id: String,
--     published_at: NaiveDateTime,
--     image: String,
--     listen: bool,
-- }
--
CREATE TABLE IF NOT EXISTS episodes(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    channel_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    yt_id TEXT NOT NULL,
    published_at DATETIME NOT NULL,
    duration TEXT NOT NULL,
    image TEXT NOT NULL,
    listen BOOLEAN NOT NULL,
    UNIQUE(channel_id, yt_id)
);
