//! Surus: a statically typed embedded DSL for PostgreSQL queries.
//!
//! Tables, columns and types come from definitions generated from a database
//! schema (see `tools/schemagen`). Queries built with the DSL form a typed
//! syntax tree: SQL types, nullability, result rows and the sources each
//! expression references are tracked in Rust types, so that invalid queries
//! fail to compile. [`Statement::compile`] renders a query as SQL text with
//! bound parameters.
//!
//! ```ignore
//! users
//!     .filter(users.name.eq(name))
//!     .select((users.id, users.name))
//!     .order_by(users.name.asc())
//!     .compile()
//! ```

// Query types encode their sources and result rows, so they are complex by
// design; expression methods consume `self`, as builders do.
#![allow(clippy::type_complexity, clippy::wrong_self_convention)]

mod catalog;
mod dml;
mod expr;
mod functions;
pub mod op;
mod query;
mod render;
mod table;
mod types;

pub use dml::*;
pub use expr::*;
pub use functions::*;
pub use query::*;
pub use render::Compiled;
pub use table::Constraint;
pub use types::*;
