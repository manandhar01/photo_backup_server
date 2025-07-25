CREATE TABLE media_metadata (
    id serial PRIMARY KEY NOT NULL,
    uuid uuid NOT NULL DEFAULT uuid_generate_v4(),
    media_id integer NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    original_filename varchar,
    mime_type varchar,
    size bigint,
    width integer,
    height integer,
    hash varchar,
    camera_make varchar,
    camera_model varchar,
    focal_length varchar,
    aperture varchar,
    taken_at timestamp,
    duration double precision,
    frame_rate real,
    video_codec varchar,
    audio_codec varchar,
    video_bitrate varchar,
    audio_bitrate varchar,
    sample_rate varchar,
    created_at timestamp WITH time zone DEFAULT NOW(),
    updated_at timestamp WITH time zone DEFAULT NOW(),
    deleted_at timestamp WITH time zone,
    created_by integer REFERENCES users(id),
    updated_by integer REFERENCES users(id)
);
