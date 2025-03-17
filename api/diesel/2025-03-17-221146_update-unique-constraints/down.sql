-- This file should undo anything in `up.sql`
-- Drop the partial unique index
DROP INDEX users_username_active_idx;

-- Recreate the original unique constraint on username
ALTER TABLE users ADD CONSTRAINT users_username_key UNIQUE (username);
