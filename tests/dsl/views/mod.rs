mod dsl;

use crate::support::{Case, rnd};

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

    case.assert_same(
        "SELECT name, group_name FROM users_with_groups ORDER BY name",
        dsl::select_from_view,
    );
}

#[test]
fn insert_through_updatable_view() {
    let mut case = Case::new();
    let group = seed(&mut case);
    let name = rnd::text();

    case.assert_same(
        &format!(
            "INSERT INTO grouped_users (name, group_id)
             VALUES ('{name}', (SELECT id FROM groups WHERE name = '{group}'))
             RETURNING name, email, group_id IS NOT NULL"
        ),
        || dsl::insert_through_updatable_view(&name, &group),
    );
}

#[test]
fn check_option_violation() {
    let mut case = Case::new();
    seed(&mut case);
    let name = rnd::text();

    case.assert_same_error(
        &format!("INSERT INTO grouped_users (name) VALUES ('{name}') RETURNING name"),
        || dsl::check_option_violation(&name),
    );
}
