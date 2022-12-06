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
    description TEXT NOT NULL,
    url TEXT NOT NULL UNIQUE,
    last DATETIME NOT NULL
);
