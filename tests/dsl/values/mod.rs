mod dsl;

use crate::support::{Case, compile, rnd, sql};
use surus::Query;
use surus::types::{Int4, Text};

#[test]
fn standalone() {
    let mut case = Case::new();
    let first = rnd::int();
    let second = rnd::text();

    let expected: Query<(Int4, Text), (Int4, Text)> = sql("VALUES ($1, $2), ($1 + 1, $2)");
    case.assert_same(expected, compile(dsl::standalone), (first, &second));
}

#[test]
fn in_from_joined_with_table() {
    let mut case = Case::new();
    let first = rnd::text();
    let second = rnd::text();

    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('{first}'), ('{second}'), ('{}')",
        rnd::text()
    ));

    let expected: Query<(Text, Text), (Text, Text)> = sql("SELECT users.name, v.label
         FROM users
         JOIN (VALUES ($1, 'first'), ($2, 'second')) AS v(name, label)
             ON v.name = users.name
         ORDER BY v.label");
    case.assert_same(
        expected,
        compile(dsl::in_from_joined_with_table),
        (&first, &second),
    );
}
