DROP SCHEMA IF EXISTS app CASCADE;
DROP TABLE IF EXISTS "Order" CASCADE;

CREATE SCHEMA app;

CREATE TABLE app.users (
    id   bigserial PRIMARY KEY,
    name text NOT NULL
);

CREATE TABLE "Order" (
    "Id"         bigserial PRIMARY KEY,
    "user"       text NOT NULL,
    "select"     int4 NOT NULL,
    "line total" numeric
);
