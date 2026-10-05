# Surus: Typed PostgreSQL DSL

A specification-driven coding-agent experiment focused on building a statically
typed embedded DSL for PostgreSQL in Rust.

The project starts from PostgreSQL schema metadata—tables, columns, types, keys,
constraints, and related catalog information—and exposes that schema through
generated typed definitions.

Queries are constructed through an embedded DSL, represented internally as a
typed query AST, and compiled into PostgreSQL SQL with bound parameters.

The main goals are:

- PostgreSQL-specific semantics rather than database portability
- schema-aware, compile-time type safety
- typed expressions, projections, joins, and query results
- compile-time rejection of invalid queries where practical
- support for PostgreSQL-specific features
- clean separation between query construction, SQL generation, and execution

The repository is also an experiment in specification-driven software
development with coding agents.

Rather than treating the coding agent as a coding tool, the project uses a
written specification as the primary source of intent. The coding agent is
expected to derive implementation decisions from that specification, produce
tests, refine the design, and keep the implementation aligned with the stated
invariants.

The broader question is whether a sufficiently precise specification can serve
as an effective interface between a human designer and a coding agent when
implementing a non-trivial, strongly typed system.

# Initial prompt

The prompt given to the agent (`docs/initial_prompt.md`):

```
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
```

# Running the experiment

Start the VM from the repository root; the disk images are created on first run
and the content of the current commit is exported to `/workspace/surus` inside
it, without git history:

```
nix run .#nixosConfigurations.agent.config.microvm.declaredRunner
```

Log in and launch the agent with the prompt from `docs/initial_prompt.md`
(password `agent`):

```
ssh -p 2222 agent@localhost

# claude
claude setup-token # auth and keep the token

export CLAUDE_CODE_OAUTH_TOKEN="PUT_THE_TOKEN_HERE"
export SURUS_TEST_DATABASE_URL='postgres:///agent?host=/run/postgresql&user=agent'

# claude-sonnet-5|claude-opus-5-5|claude-fable-5-1
export CLAUDE_MODEL=claude-sonnet-5
# low|medium|high|xhigh|max
export CLAUDE_EFFORT=medium

claude --version > /workspace/claude.version

cd /workspace/surus

nohup claude -p "$(cat docs/initial_prompt.md)" \
  --model "$CLAUDE_MODEL" \
  --permission-mode bypassPermissions \
  --max-turns 300 \
  --output-format stream-json --verbose \
  --effort "$CLAUDE_EFFORT" \
  > /workspace/run.jsonl 2> /workspace/run.err < /dev/null &

tail -f /workspace/run.jsonl # follow progress; Ctrl-C stops tail, not the run
```

The run survives a dropped SSH session. It is finished when
`pgrep -x claude` prints nothing.

The agent works in the VM's checkout and may leave changes uncommitted. From the
host, pack the working copy inside the VM and stream it over SSH, skipping build
output:

```
ssh -p 2222 agent@localhost 'tar -C /workspace -czf - --exclude=surus/target surus' > surus-agent-run.tar.gz
ssh -p 2222 agent@localhost 'cat /workspace/run.jsonl' > run.jsonl
ssh -p 2222 agent@localhost 'cat /workspace/claude.version' > claude.version
ssh -p 2222 agent@localhost 'cat /workspace/run.err' > run.err
```

Then stop the VM; only `/workspace` survives a restart, and nothing is written
back to the host checkout.

Verify that the agent left the specification and the tests untouched, passing
the commit the VM was started from:

```
./check-protected.sh surus-agent-run.tar.gz <commit>
```

`run-experiment.sh` records the commit in `commit` and the result in
`protected.txt`.
