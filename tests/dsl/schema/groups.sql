DROP TABLE IF EXISTS groups CASCADE;

CREATE TABLE groups (
    id        bigserial PRIMARY KEY,
    name      text NOT NULL,
    parent_id int8 REFERENCES groups (id)
);
