CREATE TABLE login_activity (
    id serial PRIMARY KEY NOT NULL,
    user_id integer REFERENCES users(id),
    email varchar NOT NULL,
    success boolean DEFAULT false,
    ip_address varchar,
    user_agent varchar,
    created_at timestamp WITH time zone DEFAULT NOW()
);
