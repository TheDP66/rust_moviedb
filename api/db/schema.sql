CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS films
(
    id uuid DEFAULT uuid_generate_v4() NOT NULL CONTRAINT films_pkey PRIMARY KEY,
    title text NOT NULL,
    director text NOT NULL,
    year smallint NOT NULL,
    poster text NOT NULL,
    created_at timestamp with time zone default CURRENT_TIMESTAMP,
    updated_at timestamp with time zone
)