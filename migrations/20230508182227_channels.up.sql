-- pub struct Channel {
--     id: i64,
--     url: String,
--     title: String,
--     description: String,
--     image: String,
--     first: NaiveDateTime,
-- }
CREATE TABLE IF NOT EXISTS channels(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    url TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL UNIQUE,
    description TEXT,
    image TEXT,
    first DATETIME NOT NULL
)
