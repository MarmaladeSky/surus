mod dsl;

use crate::support::{Case, compile, rnd, sql};
use surus::Query;
use surus::types::{Int8, Nullable, Text};

fn seed(case: &mut Case) {
    let big = rnd::text();
    let small = rnd::text();
    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{big}'), ('{small}');
         INSERT INTO users (name, email, group_id)
             SELECT '{}', '{}', id FROM groups WHERE name = '{big}'
             UNION ALL SELECT '{}', NULL, id FROM groups WHERE name = '{big}'
             UNION ALL SELECT '{}', '{}', id FROM groups WHERE name = '{small}'
             UNION ALL SELECT '{}', NULL, NULL;",
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));
}

#[test]
fn desc() {
    let mut case = Case::new();
    seed(&mut case);

    let expected: Query<(), (Text,)> = sql("SELECT name FROM users ORDER BY name DESC");
    case.assert_same(expected, compile(dsl::desc), ());
}

#[test]
fn nulls_first() {
    let mut case = Case::new();
    seed(&mut case);

    let expected: Query<(), (Text, Nullable<Text>)> =
        sql("SELECT name, email FROM users ORDER BY email NULLS FIRST, name");
    case.assert_same(expected, compile(dsl::nulls_first), ());
}

#[test]
fn desc_nulls_last() {
    let mut case = Case::new();
    seed(&mut case);

    let expected: Query<(), (Text, Nullable<Text>)> =
        sql("SELECT name, email FROM users ORDER BY email DESC NULLS LAST, name");
    case.assert_same(expected, compile(dsl::desc_nulls_last), ());
}

#[test]
fn multiple_keys() {
    let mut case = Case::new();
    seed(&mut case);

    let expected: Query<(), (Nullable<Int8>, Text)> =
        sql("SELECT group_id, name FROM users ORDER BY group_id DESC, name ASC");
    case.assert_same(expected, compile(dsl::multiple_keys), ());
}

#[test]
fn by_expression() {
    let mut case = Case::new();
    seed(&mut case);

    let expected: Query<(), (Text, Nullable<Text>)> =
        sql("SELECT name, email FROM users ORDER BY email IS NULL, name");
    case.assert_same(expected, compile(dsl::by_expression), ());
}

#[test]
fn by_position() {
    let mut case = Case::new();
    seed(&mut case);

    let expected: Query<(), (Nullable<Int8>, Text)> =
        sql("SELECT group_id, name FROM users ORDER BY 1, 2");
    case.assert_same(expected, compile(dsl::by_position), ());
}

#[test]
fn collate_c() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('alpha_{}'), ('Beta_{}'), ('gamma_{}'), ('Delta_{}')",
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    let expected: Query<(), (Text,)> = sql("SELECT name FROM users ORDER BY name COLLATE \"C\"");
    case.assert_same(expected, compile(dsl::collate_c), ());
}

#[test]
fn using_operator() {
    let mut case = Case::new();
    seed(&mut case);

    let expected: Query<(), (Nullable<Int8>, Text)> =
        sql("SELECT group_id, name FROM users ORDER BY group_id USING >, name USING <");
    case.assert_same(expected, compile(dsl::using_operator), ());
}
