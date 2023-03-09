ALTER TABLE todos ADD PRIMARY KEY (id);

CREATE TABLE categories(
  id SERIAL PRIMARY KEY,
  label VARCHAR NOT NULL,
  -- This is hex. I should make hex validation in the db
  -- https://dba.stackexchange.com/questions/271807/storing-hex-colors-in-a-column-in-postgres
  color VARCHAR(7) NOT NULL
);

ALTER TABLE todos
ADD COLUMN category_id INTEGER;

ALTER TABLE todos
    ADD CONSTRAINT fk_todos_categories FOREIGN KEY (category_id) REFERENCES categories (id);