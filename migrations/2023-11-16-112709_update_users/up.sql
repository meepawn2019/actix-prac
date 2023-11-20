-- Your SQL goes here
ALTER TABLE users ADD CONSTRAINT username_unique UNIQUE (username);
ALTER TABLE users ALTER COLUMN password TYPE VARCHAR;