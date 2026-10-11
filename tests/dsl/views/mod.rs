mod dsl;

use crate::support::{Case, compile, rnd, sql};
use surus::Query;
use surus::types::{Bool, Nullable, Text};

fn seed(case: &mut Case) -> String {
    let group = rnd::text();
    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{group}');
         INSERT INTO users (name, group_id)
             SELECT '{}', id FROM groups WHERE name = '{group}'
             UNION ALL SELECT '{}', NULL;",
        rnd::text(),
        rnd::text(),
    ));
    group
}

#[test]
fn select_from_view() {
    let mut case = Case::new();
    seed(&mut case);

    let expected: Query<(), (Text, Text)> =
        sql("SELECT name, group_name FROM users_with_groups ORDER BY name");
    case.assert_same(expected, compile(dsl::select_from_view), ());
}

#[test]
fn insert_through_updatable_view() {
    let mut case = Case::new();
    let group = seed(&mut case);
    let name = rnd::text();

    let expected: Query<(Text, Text), (Text, Nullable<Text>, Bool)> =
        sql("INSERT INTO grouped_users (name, group_id)
         VALUES ($1, (SELECT id FROM groups WHERE name = $2))
         RETURNING name, email, group_id IS NOT NULL");
    case.assert_same(
        expected,
        compile(dsl::insert_through_updatable_view),
        (&name, &group),
    );
}

#[test]
fn check_option_violation() {
    let mut case = Case::new();
    seed(&mut case);
    let name = rnd::text();

    let expected: Query<(Text,), (Text,)> =
        sql("INSERT INTO grouped_users (name) VALUES ($1) RETURNING name");
    case.assert_same_error(expected, compile(dsl::check_option_violation), (&name,));
}
