-- Add up migration script here
CREATE TABLE IF NOT EXISTS ingredient (
    uuid VARCHAR(16) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    amount DOUBLE NOT NULL,
    unit VARCHAR(10) NOT NULL,
    CONSTRAINT ingredient_unique unique (uuid)
);