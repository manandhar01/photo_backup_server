CREATE extension IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    id serial PRIMARY KEY NOT NULL,
    uuid uuid NOT NULL DEFAULT (uuid_generate_v4()),
    email varchar NOT NULL UNIQUE,
    username varchar NOT NULL UNIQUE,
    PASSWORD varchar,
    created_at timestamp WITH time zone DEFAULT NOW(),
    updated_at timestamp WITH time zone DEFAULT NOW(),
    deleted_at timestamp WITH time zone,
    created_by integer REFERENCES users(id),
    updated_by integer REFERENCES users(id)
);

INSERT INTO
    users (id, email, username)
VALUES
    (
        1,
        'superuser@system.com',
        'superuser'
    ),
    (
        2,
        'internaluser@system.com',
        'internaluser'
    );

SELECT
    setval(pg_get_serial_sequence('users', 'id'), 100, TRUE);
