-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
	IF NOT EXISTS users (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    active BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP
         WITH
           TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP
         WITH
           TIME ZONE DEFAULT NOW()
);
