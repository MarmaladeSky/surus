DROP TABLE IF EXISTS users CASCADE;

CREATE TABLE users (
    id       bigserial PRIMARY KEY,
    name     text NOT NULL,
    email    text,
    group_id int8 REFERENCES groups (id)
);

CREATE UNIQUE INDEX users_email_key ON users (email) WHERE email IS NOT NULL;
