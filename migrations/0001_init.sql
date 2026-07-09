-- Initial schema: users table backing src/models/user.rs::User

CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS users (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email           VARCHAR(255) NOT NULL UNIQUE,
    username        VARCHAR(255) NOT NULL UNIQUE,
    password_hash   VARCHAR(255) NOT NULL DEFAULT '',
    avatar_url      VARCHAR(1024),
    oauth_provider  VARCHAR(50),
    oauth_id        VARCHAR(255),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Used by User::find_or_create_oauth_user to look up an existing OAuth account
CREATE UNIQUE INDEX IF NOT EXISTS idx_users_oauth_provider_id
    ON users (oauth_provider, oauth_id)
    WHERE oauth_provider IS NOT NULL AND oauth_id IS NOT NULL;
