-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR(16) NOT NULL UNIQUE,
  password VARCHAR(64) NOT NULL
);

CREATE INDEX users_id_index ON users(id);