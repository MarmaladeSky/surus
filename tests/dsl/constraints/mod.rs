mod dsl;

use crate::support::{Case, compile, rnd, sql};
use surus::Query;
use surus::types::{Int8, Text};

#[test]
fn check_violation() {
    let mut case = Case::new();
    let name = rnd::text();

    let expected: Query<(Text,), (Text,)> =
        sql("INSERT INTO products (name, price_cents) VALUES ($1, -1) RETURNING name");
    case.assert_same_error(expected, compile(|| dsl::check_violation(-1)), (&name,));
}

#[test]
fn unique_violation() {
    let mut case = Case::new();
    let batch = rnd::text();
    let name = rnd::text();
    case.exec(&format!(
        "INSERT INTO tickets (batch, name) VALUES ('{batch}', '{name}')"
    ));

    let expected: Query<(Text, Text), (Int8,)> =
        sql("INSERT INTO tickets (batch, name) VALUES ($1, $2) RETURNING id");
    case.assert_same_error(expected, compile(dsl::unique_violation), (&batch, &name));
}

#[test]
fn on_conflict_on_constraint() {
    let mut case = Case::new();
    let batch = rnd::text();
    let name = rnd::text();
    case.exec(&format!(
        "INSERT INTO tickets (batch, name) VALUES ('{batch}', '{name}')"
    ));

    let expected: Query<(Text, Text), (Text, Text)> =
        sql("INSERT INTO tickets (batch, name) VALUES ($1, $2)
         ON CONFLICT ON CONSTRAINT tickets_batch_name_key DO UPDATE SET name = tickets.name || '!'
         RETURNING batch, name");
    case.assert_same(
        expected,
        compile(dsl::on_conflict_on_constraint),
        (&batch, &name),
    );
}
