-- Your SQL goes here

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  encrypted_password VARCHAR NOT NULL
);

CREATE UNIQUE INDEX index_users_on_username ON users USING btree (username);
