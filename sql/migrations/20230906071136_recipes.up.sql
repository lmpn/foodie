-- Add up migration script here
CREATE TABLE IF NOT EXISTS recipe (
    uuid VARCHAR(16) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    image VARCHAR(255),
    method VARCHAR(255),
    CONSTRAINT recipe_unique unique (uuid)
)