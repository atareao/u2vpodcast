CREATE TABLE IF NOT EXISTS episodes(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    channel_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    yt_id TEXT NOT NULL,
    webpage_url TEXT NOT NULL,
    published_at DATETIME NOT NULL,
    duration TEXT NOT NULL,
    image TEXT NOT NULL DEFAULT '',
    listen BOOLEAN NOT NULL DEFAULT FALSE,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    UNIQUE(channel_id, yt_id)
);
