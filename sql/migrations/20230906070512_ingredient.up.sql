-- Add up migration script here
CREATE TABLE IF NOT EXISTS ingredient (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(255) NOT NULL,
    amount DOUBLE NOT NULL,
    unit VARCHAR(10) NOT NULL
);