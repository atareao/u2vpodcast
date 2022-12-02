-- pub struct Channel {
--     id: i64,
--     yt_id: String,
--     title: String,
--     last: NaiveDateTime,
-- }
--
CREATE TABLE IF NOT EXISTS channels(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    path TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    yt_id TEXT NOT NULL UNIQUE,
    last DATETIME NOT NULL
);
