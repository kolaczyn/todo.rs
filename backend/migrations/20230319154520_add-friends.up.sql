CREATE TABLE friends (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  friend_id INTEGER NOT NULL,
  CONSTRAINT fk_friends_users FOREIGN KEY (user_id) REFERENCES users (id),
  CONSTRAINT fk_friends_friends FOREIGN KEY (friend_id) REFERENCES users (id)
);

