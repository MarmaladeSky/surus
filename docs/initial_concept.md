# Typed PostgreSQL DSL — Initial Specification

## 1. Goal

The project provides a statically typed embedded DSL for constructing PostgreSQL
queries in a host language such as Rust.

The DSL must allow application code to construct queries that:

- reference database relations and columns through generated typed definitions;
- are validated as far as possible by the host-language type system;
- represent PostgreSQL semantics rather than merely concatenate SQL strings;
- compile into valid PostgreSQL SQL;
- can later be executed through a separate database execution layer.

The DSL is PostgreSQL-specific. Database portability is not a design goal.

## 2. Schema Model

The project obtains database schema metadata from a PostgreSQL instance.

Relevant metadata includes:

- schemas;
- tables and views;
- columns;
- PostgreSQL column types;
- nullability;
- primary keys;
- unique constraints;
- foreign keys;
- check constraints where useful;
- indexes where relevant to DSL semantics;
- enums;
- domains;
- composite types;
- functions and operators where supported.

This metadata is converted into generated host-language definitions forming the
statically known database schema.

For example, conceptually:

```text
users.id       : Column<Int8, NotNull>
users.name     : Column<Text, NotNull>
users.email    : Column<Text, Nullable>
users.group_id : Column<Int8, Nullable>
```

The generated schema definitions are the interface between PostgreSQL metadata
and the DSL type system.

## 3. DSL Model

The DSL is an embedded, receiver-oriented, fluent API.

Example syntax may look conceptually like:

```rust
users
    .filter(users.age.gt(18))
    .select((users.id, users.name))
    .order_by(users.name.asc())
```

The API syntax is only a frontend.

Every DSL expression must construct a typed internal representation of the
PostgreSQL query.

The DSL must not generate SQL incrementally through string concatenation.

## 4. Typed Query Representation

Queries and expressions are represented by a typed AST.

Conceptually:

```text
Select<
    From<Users>,
    Projection<(Users.Id, Users.Name)>,
    Where<Gt<Users.Age, Int4>>,
    OrderBy<Asc<Users.Name>>
>
```

The AST should preserve enough semantic information to:

1. reject invalid constructions at compile time;
1. determine the query result type;
1. render valid PostgreSQL SQL;
1. perform later transformations or optimizations without reparsing SQL text.

## 5. Type-Safety Requirements

The host-language type system should reject invalid queries wherever the
required information is statically available.

The DSL should statically validate at least:

- table and column existence;
- column ownership and query scope;
- expression input and output types;
- operator operand compatibility;
- function argument types;
- boolean requirements for `WHERE`, `HAVING`, and join predicates;
- comparison compatibility;
- nullability propagation;
- projection/result shape;
- `INSERT` value-to-column compatibility;
- `UPDATE` assignment compatibility;
- join relationships where relevant;
- aggregate expression rules where feasible;
- `GROUP BY` compatibility where feasible;
- `RETURNING` result types;
- CTE output types;
- set-operation compatibility for `UNION`, `INTERSECT`, and `EXCEPT`.

A query accepted by the DSL type system should not fail because of a structural
or type error that could have been determined from the known PostgreSQL schema.

## 6. PostgreSQL Type System

The DSL should model PostgreSQL types explicitly.

Examples include:

```text
Bool
Int2
Int4
Int8
Numeric
Float4
Float8
Text
Varchar
Uuid
Date
Timestamp
Timestamptz
Json
Jsonb
Array<T>
Range<T>
Enum<E>
Nullable<T>
```

The model should distinguish:

```text
Column<T>
Expression<T>
Parameter<T>
Literal<T>
Query<Row>
```

where appropriate.

Host-language runtime types and PostgreSQL SQL types must not be assumed to be
identical.

Explicit mapping between them is required.

## 7. PostgreSQL Semantics

The DSL should progressively support PostgreSQL-specific constructs rather than
restricting itself to portable SQL.

Target constructs include:

- `SELECT`;
- `INSERT`;
- `UPDATE`;
- `DELETE`;
- `MERGE`;
- joins;
- subqueries;
- CTEs;
- recursive CTEs;
- aggregates;
- `GROUP BY`;
- `HAVING`;
- window functions;
- `FILTER`;
- `ORDER BY`;
- `DISTINCT`;
- `DISTINCT ON`;
- `LIMIT` / `OFFSET`;
- set operations;
- `RETURNING`;
- `ON CONFLICT`;
- `LATERAL`;
- arrays;
- row expressions;
- JSON/JSONB expressions;
- range and multirange expressions;
- PostgreSQL casts;
- PostgreSQL operators;
- locking clauses;
- PostgreSQL functions.

Coverage may be incremental, but the architecture must not prevent eventual
representation of most modern PostgreSQL query grammar.

## 8. Parameters and SQL Generation

Runtime values must normally be represented as SQL parameters rather than
interpolated into generated SQL.

For example:

```rust
users.filter(users.age.gt(param(18)))
```

may compile to:

```sql
SELECT ...
FROM users
WHERE users.age > $1
```

with a separate parameter list:

```text
$1 : Int4 = 18
```

SQL generation therefore produces at least:

```text
CompiledQuery {
    sql,
    parameters,
    result_type
}
```

SQL rendering is a backend stage operating on the typed AST.

## 9. Separation of Concerns

The project should separate four major components:

```text
PostgreSQL catalog
        ↓
Schema Introspection
        ↓
Generated Typed Schema
        ↓
Typed PostgreSQL DSL
        ↓
Typed Query AST
        ↓
PostgreSQL SQL Renderer
        ↓
SQL + Parameters
```

Database execution is a separate concern:

```text
SQL + Parameters
        ↓
PostgreSQL Driver
        ↓
Result decoding
```

The DSL must not depend fundamentally on a particular PostgreSQL client library.

## 10. Scope Boundary

The project does not attempt to prove that every SQL statement accepted by every
PostgreSQL installation can be represented statically.

PostgreSQL is extensible through:

- extensions;
- custom types;
- custom functions;
- custom operators;
- domains;
- user-defined casts.

The practical guarantee is instead:

> Given a known PostgreSQL schema and type environment, every query successfully
> constructed through the typed DSL should be structurally and type-correct
> according to that known environment and should compile into valid PostgreSQL
> SQL.

Unknown or user-defined PostgreSQL functionality may be incorporated through
generated metadata or explicit extension APIs.

## 11. Primary Design Principle

The project should be treated primarily as:

> **a statically typed PostgreSQL query AST with an ergonomic embedded DSL for
> constructing it**

rather than as:

> **a SQL string builder with type annotations**

Type safety and semantic correctness belong in the query representation itself.
SQL generation is only the final serialization step.
