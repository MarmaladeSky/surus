mod dsl;

use crate::support::{Case, rnd};

#[test]
fn standalone() {
    let mut case = Case::new();
    let first = rnd::int();
    let second = rnd::text();

    case.assert_same(
        &format!(
            "VALUES ({first}, '{second}'), ({}, '{}')",
            first + 1,
            second
        ),
        || dsl::standalone(first, &second),
    );
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

    case.assert_same(
        &format!(
            "SELECT users.name, v.label
             FROM users
             JOIN (VALUES ('{first}', 'first'), ('{second}', 'second')) AS v(name, label)
                 ON v.name = users.name
             ORDER BY v.label"
        ),
        || dsl::in_from_joined_with_table(&first, &second),
    );
}
