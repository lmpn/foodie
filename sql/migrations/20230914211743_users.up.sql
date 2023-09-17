-- Add up migration script here
CREATE TABLE "users" (
    id VARCHAR(16) PRIMARY KEY NOT NULL,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(100) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    photo VARCHAR NOT NULL DEFAULT 'default.png',
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    CONSTRAINT users_unique unique (id),
    CONSTRAINT users_email_unique unique (email)
)