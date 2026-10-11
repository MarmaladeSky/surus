mod dsl;

use crate::support::{Case, compile, rnd, sql};
use surus::Query;
use surus::types::{Bool, Int8, Nullable, Text};

#[test]
fn by_name() {
    let mut case = Case::new();
    let doomed = rnd::text();
    let kept = rnd::text();

    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('{doomed}'), ('{kept}')"
    ));

    let expected: Query<(Text,), (Text,)> = sql("DELETE FROM users WHERE name = $1 RETURNING name");
    case.assert_same(expected, compile(dsl::by_name), (&doomed,));
}

#[test]
fn delete_using() {
    let mut case = Case::new();
    let doomed_group = rnd::text();

    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{doomed_group}'), ('{}');
         INSERT INTO users (name, group_id)
             SELECT '{}', id FROM groups WHERE name = '{doomed_group}'
             UNION ALL SELECT '{}', id FROM groups WHERE name <> '{doomed_group}';",
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    let expected: Query<(Text,), (Text,)> = sql("DELETE FROM users USING groups
         WHERE groups.id = users.group_id AND groups.name = $1
         RETURNING users.name");
    case.assert_same(expected, compile(dsl::delete_using), (&doomed_group,));
}

#[test]
fn returning_table_star() {
    let mut case = Case::new();
    let group = rnd::text();
    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{group}');
         INSERT INTO users (name, email, group_id)
             SELECT '{}', '{}', id FROM groups WHERE name = '{group}'
             UNION ALL SELECT '{}', NULL, NULL;",
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    let expected: Query<(Text,), (Int8, Text, Nullable<Text>, Nullable<Int8>, Text)> =
        sql("DELETE FROM users USING groups
         WHERE groups.id = users.group_id AND groups.name = $1
         RETURNING users.*, groups.name AS group_name");
    case.assert_same_named(expected, compile(dsl::returning_table_star), (&group,));
}

#[test]
fn returning_old() {
    let mut case = Case::new();
    let name = rnd::text();
    case.exec(&format!(
        "INSERT INTO users (name, email) VALUES ('{name}', '{}')",
        rnd::text()
    ));

    let expected: Query<(Text,), (Text, Nullable<Text>, Bool)> =
        sql("DELETE FROM users WHERE name = $1 RETURNING old.name, old.email, new.name IS NULL");
    case.assert_same(expected, compile(dsl::returning_old), (&name,));
}

#[test]
fn delete_using_subquery() {
    let mut case = Case::new();
    let root = rnd::text();
    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{root}');
         INSERT INTO groups (name, parent_id) SELECT '{}', id FROM groups WHERE name = '{root}';
         INSERT INTO users (name, group_id)
             SELECT '{}', id FROM groups WHERE name = '{root}'
             UNION ALL SELECT '{}', id FROM groups WHERE parent_id IS NOT NULL
             UNION ALL SELECT '{}', NULL;",
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    let expected: Query<(), (Text,)> = sql(
        "DELETE FROM users WHERE group_id IN (SELECT id FROM groups WHERE parent_id IS NULL) RETURNING name",
    );
    case.assert_same(expected, compile(dsl::delete_using_subquery), ());
}
