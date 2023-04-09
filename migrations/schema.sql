-- Add migration script here
CREATE TABLE schedule(
  id uuid PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  email TEXT NOT NULL UNIQUE,
  active BOOLEAN DEFAULT FALSE
);
