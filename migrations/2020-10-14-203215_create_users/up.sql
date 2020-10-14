-- Your SQL goes here

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  encrypted_password VARCHAR NOT NULL
);
