DROP TYPE IF EXISTS mood CASCADE;
DROP DOMAIN IF EXISTS positive_int CASCADE;
DROP TYPE IF EXISTS price_tag CASCADE;

CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');

CREATE DOMAIN positive_int AS int4 CHECK (VALUE > 0);

CREATE TYPE price_tag AS (
    amount   numeric,
    currency text
);
