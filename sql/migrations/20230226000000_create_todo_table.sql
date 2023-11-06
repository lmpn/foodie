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
);

CREATE TABLE IF NOT EXISTS user_permissions (
    user_id  INTEGER NOT NULL,
    token    TEXT NOT NULL
);

-- INSERT INTO users (id, anonymous, username, password) 
-- SELECT 0, true, 'Guest', ''
-- ON CONFLICT(id) DO UPDATE SET
--     anonymous = EXCLUDED.anonymous,
--     username = EXCLUDED.username;


CREATE TABLE IF NOT EXISTS todos (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id    INTEGER NOT NULL,
  title      TEXT NOT NULL,
  completed  BOOLEAN,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
  -- FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);
