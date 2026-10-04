mod dsl;

use crate::support::{Case, rnd};

#[test]
fn single_row() {
    let mut case = Case::new();
    let name = rnd::text();

    case.exec(&format!("INSERT INTO users (name) VALUES ('{name}')"));

    case.assert_same(
        "SELECT id, name, email, group_id FROM users",
        dsl::single_row,
    );
}

fn seed_two_groups(case: &mut Case) {
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
fn distinct_group_ids() {
    let mut case = Case::new();
    seed_two_groups(&mut case);

    case.assert_same(
        "SELECT DISTINCT group_id FROM users ORDER BY group_id",
        dsl::distinct_group_ids,
    );
}

#[test]
fn distinct_on_first_user_per_group() {
    let mut case = Case::new();
    seed_two_groups(&mut case);

    case.assert_same(
        "SELECT DISTINCT ON (group_id) group_id, name FROM users ORDER BY group_id, name",
        dsl::distinct_on_first_user_per_group,
    );
}

#[test]
fn limit_offset() {
    let mut case = Case::new();
    seed_two_groups(&mut case);

    case.assert_same(
        "SELECT name FROM users ORDER BY name LIMIT 2 OFFSET 1",
        || dsl::limit_offset(2, 1),
    );
}

#[test]
fn fetch_first_rows_only() {
    let mut case = Case::new();
    seed_two_groups(&mut case);

    case.assert_same(
        "SELECT name FROM users ORDER BY name OFFSET 1 ROWS FETCH FIRST 2 ROWS ONLY",
        || dsl::fetch_first_rows_only(2, 1),
    );
}

#[test]
fn fetch_first_with_ties() {
    let mut case = Case::new();
    seed_two_groups(&mut case);

    case.assert_same(
        "SELECT group_id FROM users ORDER BY group_id FETCH FIRST 1 ROWS WITH TIES",
        || dsl::fetch_first_with_ties(1),
    );
}

#[test]
fn aliases() {
    let mut case = Case::new();
    seed_two_groups(&mut case);

    case.assert_same_named(
        "SELECT u.name AS user_name, g.name AS group_name
         FROM users AS u
         JOIN groups AS g ON g.id = u.group_id
         ORDER BY user_name",
        dsl::aliases,
    );
}

fn seed_tickets(case: &mut Case) -> (String, String) {
    let batch = rnd::text();
    let locked = rnd::text();
    case.exec_committed(&format!(
        "INSERT INTO tickets (batch, name) VALUES ('{batch}', '{locked}'), ('{batch}', '{}'), ('{batch}', '{}')",
        rnd::text(),
        rnd::text(),
    ));
    (batch, locked)
}

#[test]
fn for_update() {
    let mut case = Case::new();
    let (batch, _) = seed_tickets(&mut case);

    case.assert_same(
        &format!("SELECT name FROM tickets WHERE batch = '{batch}' ORDER BY name FOR UPDATE"),
        || dsl::for_update(&batch),
    );
}

#[test]
fn for_share_alongside_other_share() {
    let mut case = Case::new();
    let (batch, locked) = seed_tickets(&mut case);
    let mut other = Case::new();
    other.exec(&format!(
        "SELECT 1 FROM tickets WHERE name = '{locked}' FOR SHARE"
    ));

    case.assert_same(
        &format!("SELECT name FROM tickets WHERE batch = '{batch}' ORDER BY name FOR SHARE"),
        || dsl::for_share(&batch),
    );
}

#[test]
fn for_update_skip_locked() {
    let mut case = Case::new();
    let (batch, locked) = seed_tickets(&mut case);
    let mut other = Case::new();
    other.exec(&format!(
        "SELECT 1 FROM tickets WHERE name = '{locked}' FOR UPDATE"
    ));

    case.assert_same(
        &format!(
            "SELECT name FROM tickets WHERE batch = '{batch}' ORDER BY name FOR UPDATE SKIP LOCKED"
        ),
        || dsl::for_update_skip_locked(&batch),
    );
}

#[test]
fn for_update_nowait_on_locked_row() {
    let mut case = Case::new();
    let (_, locked) = seed_tickets(&mut case);
    let mut other = Case::new();
    other.exec(&format!(
        "SELECT 1 FROM tickets WHERE name = '{locked}' FOR UPDATE"
    ));

    case.assert_same_error(
        &format!("SELECT name FROM tickets WHERE name = '{locked}' FOR UPDATE NOWAIT"),
        || dsl::for_update_nowait(&locked),
    );
}

#[test]
fn star() {
    let mut case = Case::new();
    seed_two_groups(&mut case);

    case.assert_same_named("SELECT * FROM users ORDER BY name", dsl::star);
}

#[test]
fn table_star() {
    let mut case = Case::new();
    seed_two_groups(&mut case);

    case.assert_same_named(
        "SELECT u.*, g.name AS group_name FROM users u JOIN groups g ON g.id = u.group_id ORDER BY u.name",
        dsl::table_star,
    );
}

#[test]
fn without_from() {
    let mut case = Case::new();

    case.assert_same(
        "SELECT 1 + 1, 'a' || 'b', 10 > 3, CURRENT_DATE = CURRENT_DATE",
        dsl::without_from,
    );
}

#[test]
fn for_key_share_alongside_no_key_update() {
    let mut case = Case::new();
    let (batch, locked) = seed_tickets(&mut case);
    let mut other = Case::new();
    other.exec(&format!(
        "SELECT 1 FROM tickets WHERE name = '{locked}' FOR NO KEY UPDATE"
    ));

    case.assert_same(
        &format!("SELECT name FROM tickets WHERE batch = '{batch}' ORDER BY name FOR KEY SHARE"),
        || dsl::for_key_share(&batch),
    );
}

#[test]
fn for_no_key_update_skip_locked() {
    let mut case = Case::new();
    let (batch, locked) = seed_tickets(&mut case);
    let mut other = Case::new();
    other.exec(&format!(
        "SELECT 1 FROM tickets WHERE name = '{locked}' FOR NO KEY UPDATE"
    ));

    case.assert_same(
        &format!(
            "SELECT name FROM tickets WHERE batch = '{batch}' ORDER BY name FOR NO KEY UPDATE SKIP LOCKED"
        ),
        || dsl::for_no_key_update_skip_locked(&batch),
    );
}

#[test]
fn for_update_of_table() {
    let mut case = Case::new();
    seed_two_groups(&mut case);

    case.assert_same(
        "SELECT u.name, g.name FROM users u JOIN groups g ON g.id = u.group_id ORDER BY u.name FOR UPDATE OF u",
        dsl::for_update_of_table,
    );
}
