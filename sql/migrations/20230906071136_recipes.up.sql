-- Add up migration script here
CREATE TABLE IF NOT EXISTS recipe (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(255) NOT NULL,
    image VARCHAR(255),
    method VARCHAR(255)
)