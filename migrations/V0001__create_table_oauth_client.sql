CREATE TABLE IF NOT EXISTS oauth_client
(
    id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    name TEXT,
    slug TEXT,
    secret TEXT DEFAULT encode(sha256(gen_random_uuid()::text::bytea), 'hex'),
    urls _TEXT,
    scopes _TEXT NULL,
    status INT DEFAULT 1,
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_oauth_client_slug ON oauth_client using hash (slug);
CREATE INDEX IF NOT EXISTS idx_oauth_client_secret ON oauth_client using hash (secret);
