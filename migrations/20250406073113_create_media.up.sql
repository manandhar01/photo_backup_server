CREATE TABLE media (
    id serial PRIMARY KEY,
    uuid uuid NOT NULL DEFAULT gen_random_uuid(),
    user_id integer NOT NULL REFERENCES users(id),
    filename text NOT NULL,
    filepath text NOT NULL,
    media_type integer NOT NULL,
    created_at timestamptz DEFAULT NOW(),
    updated_at timestamptz DEFAULT NOW(),
    deleted_at timestamptz,
    attributes jsonb
)
