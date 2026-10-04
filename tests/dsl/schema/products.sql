DROP TABLE IF EXISTS products CASCADE;

CREATE TABLE products (
    id          int8 GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name        text NOT NULL,
    price_cents int4 NOT NULL CHECK (price_cents >= 0),
    quantity    int4 NOT NULL DEFAULT 0,
    total_cents int8 GENERATED ALWAYS AS (price_cents::int8 * quantity) STORED,
    price_with_tax int8 GENERATED ALWAYS AS (price_cents::int8 * 12 / 10) VIRTUAL
);
