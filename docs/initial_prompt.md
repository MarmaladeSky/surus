Read `README.md`, `docs/initial_concept.md`, `docs/target_environment.md` and
`docs/recommendations.md`, then implement the project described there.

Treat the specification as the primary source of truth. Make reasonable design
decisions where details are unspecified and keep the implementation idiomatic
Rust.

The test suite in `tests/dsl` already exists. Each test pairs plain SQL in
`mod.rs` with a placeholder in the sibling `dsl.rs` that returns
`SELECT NULL WHERE false`. Replace the placeholders with queries built through
the DSL; do not change the tests, the fixtures in `tests/dsl/schema`, or the
generated files under `tests/dsl/operators/catalog`.

Before implementing, split the work into a small sequence of concrete steps.
Execute them one by one, validating each step before moving to the next.

Keep the implementation as minimal as possible. Prefer the smallest design and
code surface that satisfies the specification. Avoid unnecessary abstractions,
dependencies, framework-like infrastructure, premature generalization, and
features not required by the current spec.

Never modify or rewrite the specification. If the specification is ambiguous or
incomplete, resolve it in the implementation without changing the source
documents. Never change the tests either: under `tests/dsl`, edit only the
`dsl.rs` files.

Focus on producing a small, working implementation in this repository.

Environment: disposable NixOS VM. The repository is at `/workspace/surus`; only
`/workspace` persists across reboots. Rust 1.99.0 with clippy and rustfmt is
installed. PostgreSQL 18 runs locally; `psql` connects as superuser `agent`
without a password. Tests read the connection URL from
`SURUS_TEST_DATABASE_URL`.
