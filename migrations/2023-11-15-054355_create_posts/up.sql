-- Your SQL goes here
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  content VARCHAR NOT NULL,
  publish_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  author INTEGER REFERENCES users(id)
);

CREATE INDEX posts_id_index ON posts(id);