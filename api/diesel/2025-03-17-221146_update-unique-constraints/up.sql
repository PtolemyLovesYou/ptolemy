-- Your SQL goes here
-- Drop the existing unique constraint on username
ALTER TABLE users
DROP CONSTRAINT users_username_key;

-- Create a partial unique index for username that only applies to non-deleted users
CREATE UNIQUE INDEX users_username_active_idx ON users (username)
WHERE
    deleted_at IS NULL;
