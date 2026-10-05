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

#[test]
fn union_nullable_with_not_null() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO users (name, email) VALUES ('{}', '{}'), ('{}', NULL)",
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    case.assert_same(
        "SELECT name FROM users UNION SELECT email FROM users WHERE email IS NOT NULL ORDER BY name",
        dsl::union_nullable_with_not_null,
    );
}

#[test]
fn set_op_in_from() {
    let mut case = Case::new();
    seed_overlap(&mut case);

    case.assert_same(
        "SELECT count(*) FROM (SELECT name FROM users UNION SELECT name FROM groups) s",
        dsl::set_op_in_from,
    );
}

#[test]
fn set_op_as_cte() {
    let mut case = Case::new();
    seed_overlap(&mut case);

    case.assert_same(
        "WITH names AS (SELECT name FROM users EXCEPT SELECT name FROM groups)
         SELECT name || '!' FROM names ORDER BY 1",
        dsl::set_op_as_cte,
    );
}

#[test]
fn nested_precedence() {
    let mut case = Case::new();
    let shared = rnd::text();
    case.exec(&format!(
        "INSERT INTO users (name, email) VALUES ('{}', '{shared}'), ('{}', NULL);
         INSERT INTO groups (name) VALUES ('{shared}'), ('{}');",
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    case.assert_same(
        "SELECT name FROM users
         UNION
         (SELECT name FROM groups INTERSECT SELECT email FROM users)
         ORDER BY name",
        dsl::nested_precedence,
    );
}
