-- Add up migration script here
CREATE extension IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    id serial PRIMARY KEY NOT NULL,
    uuid uuid NOT NULL DEFAULT (uuid_generate_v4()),
    email varchar NOT NULL UNIQUE,
    username varchar NOT NULL UNIQUE,
    PASSWORD varchar NOT NULL,
    created_at timestamp WITH time zone DEFAULT NOW(),
    updated_at timestamp WITH time zone DEFAULT NOW(),
    deleted_at timestamp WITH time zone
);
