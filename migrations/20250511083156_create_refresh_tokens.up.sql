CREATE TABLE refresh_tokens (
    id serial PRIMARY KEY NOT NULL,
    user_id integer REFERENCES users(id) NOT NULL,
    refresh_token varchar NOT NULL,
    expires_at timestamp WITH time zone NOT NULL,
    created_at timestamp WITH time zone DEFAULT NOW(),
    updated_at timestamp WITH time zone DEFAULT NOW(),
    deleted_at timestamp WITH time zone
);
