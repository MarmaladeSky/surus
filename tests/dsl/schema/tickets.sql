DROP TABLE IF EXISTS tickets CASCADE;

CREATE TABLE tickets (
    id    bigserial PRIMARY KEY,
    batch text NOT NULL,
    name  text NOT NULL,
    UNIQUE (batch, name)
);
