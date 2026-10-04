mod dsl;

use crate::support::{Case, rnd};

#[test]
fn user_and_group_names() {
    let mut case = Case::new();

    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('{}'), ('{}');
         INSERT INTO groups (name) VALUES ('{}');",
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    case.assert_same(
        "SELECT name FROM users UNION SELECT name FROM groups ORDER BY name",
        dsl::user_and_group_names,
    );
}

fn seed_overlap(case: &mut Case) {
    let shared = rnd::text();
    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('{shared}'), ('{}');
         INSERT INTO groups (name) VALUES ('{shared}'), ('{}');",
        rnd::text(),
        rnd::text(),
    ));
}

#[test]
fn union_all_keeps_duplicates() {
    let mut case = Case::new();
    seed_overlap(&mut case);

    case.assert_same(
        "SELECT name FROM users UNION ALL SELECT name FROM groups ORDER BY name",
        dsl::union_all_keeps_duplicates,
    );
}

#[test]
fn intersect() {
    let mut case = Case::new();
    seed_overlap(&mut case);

    case.assert_same(
        "SELECT name FROM users INTERSECT SELECT name FROM groups ORDER BY name",
        dsl::intersect,
    );
}

#[test]
fn except() {
    let mut case = Case::new();
    seed_overlap(&mut case);

    case.assert_same(
        "SELECT name FROM users EXCEPT SELECT name FROM groups ORDER BY name",
        dsl::except,
    );
}

#[test]
fn parenthesized_with_limits() {
    let mut case = Case::new();
    seed_overlap(&mut case);

    case.assert_same(
        "(SELECT name FROM users ORDER BY name LIMIT 1)
         UNION
         (SELECT name FROM groups ORDER BY name DESC LIMIT 1)
         ORDER BY name",
        dsl::parenthesized_with_limits,
    );
}
