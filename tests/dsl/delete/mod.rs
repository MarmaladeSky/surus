mod dsl;

use crate::support::{Case, rnd};

#[test]
fn by_name() {
    let mut case = Case::new();
    let doomed = rnd::text();
    let kept = rnd::text();

    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('{doomed}'), ('{kept}')"
    ));

    case.assert_same(
        &format!("DELETE FROM users WHERE name = '{doomed}' RETURNING name"),
        || dsl::by_name(&doomed),
    );
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

    case.assert_same(
        &format!(
            "DELETE FROM users USING groups
             WHERE groups.id = users.group_id AND groups.name = '{doomed_group}'
             RETURNING users.name"
        ),
        || dsl::delete_using(&doomed_group),
    );
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

    case.assert_same_named(
        &format!(
            "DELETE FROM users USING groups
             WHERE groups.id = users.group_id AND groups.name = '{group}'
             RETURNING users.*, groups.name AS group_name"
        ),
        || dsl::returning_table_star(&group),
    );
}

#[test]
fn returning_old() {
    let mut case = Case::new();
    let name = rnd::text();
    case.exec(&format!(
        "INSERT INTO users (name, email) VALUES ('{name}', '{}')",
        rnd::text()
    ));

    case.assert_same(
        &format!("DELETE FROM users WHERE name = '{name}' RETURNING old.name, old.email, new.name IS NULL"),
        || dsl::returning_old(&name),
    );
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

    case.assert_same(
        "DELETE FROM users WHERE group_id IN (SELECT id FROM groups WHERE parent_id IS NULL) RETURNING name",
        dsl::delete_using_subquery,
    );
}
