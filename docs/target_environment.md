# Target Environment

## PostgreSQL version

The DSL targets PostgreSQL 18.

## Grammar sources

- Informal synopsis: `docs/resources/synopsis.md`, extracted from
  <https://www.postgresql.org/docs/18/sql-commands.html>
- Formal grammar: `docs/resources/gram.y`, copied from
  <https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/parser/gram.y>

## Test database

Tests under `tests/dsl` expect a PostgreSQL 18 instance reachable at
`postgres://postgres:postgres@localhost:5432/postgres`, overridable through
`RSLICK_TEST_DATABASE_URL`. Fixtures in `tests/dsl/schema` are applied once
per run; each test runs inside a transaction that is rolled back.
