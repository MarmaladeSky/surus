CREATE EXTENSION IF NOT EXISTS btree_gist;

DROP TABLE IF EXISTS reservations CASCADE;

CREATE TABLE reservations (
    room   int4 NOT NULL,
    during tsrange NOT NULL,
    PRIMARY KEY (room, during WITHOUT OVERLAPS)
);
