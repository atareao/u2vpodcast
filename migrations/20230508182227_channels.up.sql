-- pub struct Channel {
--     id: i64,
--     url: String,
--     title: String,
--     description: String,
--     image: String,
--     first: NaiveDateTime,
-- }
CREATE TABLE IF NOT EXISTS channels(
    id INTEGER PRIMARY KEY AUTOINCREMENT NO NULL,
    url TEXT NOT NULL,
    title TEXT NOT NULL,
    descritpion TEXT,
    image TEXT,
    first DATETIME NOT NULL
)
