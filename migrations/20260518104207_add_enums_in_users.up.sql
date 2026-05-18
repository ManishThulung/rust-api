-- Add up migration script here
CREATE TYPE user_role AS ENUM ('user', 'admin', 'moderator');

ALTER TABLE users
ADD COLUMN role user_role NOT NULL DEFAULT 'user';