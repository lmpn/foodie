-- Add up migration script here
CREATE TABLE IF NOT EXISTS recipe (
    uuid VARCHAR(16) PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    image VARCHAR(255) NOT NULL DEFAULT 'default.png',
    method VARCHAR(255) NOT NULL DEFAULT 'no method',
    CONSTRAINT recipe_unique unique (uuid)
)