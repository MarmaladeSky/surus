# Recommendations

Design guidance that supplements the specification. The specification stays
authoritative; these notes record decisions made where it is silent.

## Operator surface

Expose a PostgreSQL operator as a Rust operator only when Rust has the same
symbol with the same meaning and the overload can return an expression. That
covers arithmetic (`+ - * / % -x`) and bitwise (`& | ^ << >> !x`).

Expose everything else as a named method:

- comparisons (`= <> < <= > >=`): `PartialEq` and `PartialOrd` must return
  `bool`, so they cannot build an AST node; use `eq`, `ne`, `lt`, `le`, `gt`,
  `ge`;
- `||`, `AND`, `OR`: not overloadable in Rust; use `concat`, `and`, `or`;
- symbols without a Rust counterpart (`@>`, `<@`, `->>`, `~~`, `<->`, ...): use
  descriptive names such as `contains`, `contained_by`, `get_text`, `like`,
  `distance`.

The rule is per operator, not per type: `int4` keeps symbolic arithmetic with
method comparisons, while `jsonb` is fully method-based.

## Complexity follows syntax

The relative complexity of DSL constructs should match the relative complexity
of the SQL constructs they produce. A simple `SELECT` must stay simple to write
and to read no matter how many other constructs the DSL supports. Adding
CTEs, window functions or `MERGE` may add types and methods, but must not add
ceremony to queries that do not use them.

## Type safety

No API accepts SQL text or values formatted into it, and no type is asserted
by the caller instead of derived by the DSL. Leave a test failing rather than
pass it otherwise; such a pass rejects the whole run.

## Evaluation

The implementation is judged on three criteria, listed by descending priority:

- readability of DSL code, assessed by a human reviewer;
- time to compile a DSL expression into SQL, as reported by the test harness;
- coverage of PostgreSQL syntax, measured by the test suite.
