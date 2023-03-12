-- Add up migration script here

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR NOT NULL UNIQUE,
  password_hash VARCHAR NOT NULL
);

ALTER TABLE todos
ADD COLUMN user_id INTEGER;

ALTER TABLE todos
    ADD CONSTRAINT fk_todos_users FOREIGN KEY (user_id) REFERENCES users (id);