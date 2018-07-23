-- Your SQL goes here
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  account VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  regulation BOOLEAN NOT NULL DEFAULT 'f'
);
CREATE TABLE img(
  id SERIAL PRIMARY KEY,
  account VARCHAR NOT NULL,
  img_url_1 TEXT,
  regulation BOOLEAN NOT NULL DEFAULT 'f'
);
