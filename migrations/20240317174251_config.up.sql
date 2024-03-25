CREATE TABLE IF NOT EXISTS config(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    key TEXT NOT NULL UNIQUE,
    value TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
INSERT INTO config (key, value) VALUES
    ('url', 'http://localhost'),
    ('port', '6996'),
    ('salt', 'salt'),
    ('pepper', 'pepper'),
    ('sleep_time', '1'),
    ('per_page', '10'),
    ('jwt_secret', 'a-secret-very-secret'),
    ('jwt_expires_in', '60m'),
    ('jwt_maxage', '60'),
    ('title', 'U2VPodcast');
