mod dsl;

use crate::support::{Case, compile, rnd, sql};
use surus::Query;
use surus::types::{Bool, Int4, Int8, Nullable, Text};

#[test]
fn single_row() {
    let mut case = Case::new();
    let name = rnd::text();

    case.exec(&format!("INSERT INTO users (name) VALUES ('{name}')"));

    let expected: Query<(), (Int8, Text, Nullable<Text>, Nullable<Int8>)> =
        sql("SELECT id, name, email, group_id FROM users");
    case.assert_same(expected, compile(dsl::single_row), ());
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

    let expected: Query<(), (Nullable<Int8>,)> =
        sql("SELECT DISTINCT group_id FROM users ORDER BY group_id");
    case.assert_same(expected, compile(dsl::distinct_group_ids), ());
}

#[test]
fn distinct_on_first_user_per_group() {
    let mut case = Case::new();
    seed_two_groups(&mut case);

    let expected: Query<(), (Nullable<Int8>, Text)> =
        sql("SELECT DISTINCT ON (group_id) group_id, name FROM users ORDER BY group_id, name");
    case.assert_same(expected, compile(dsl::distinct_on_first_user_per_group), ());
}

#[test]
fn limit_offset() {
    let mut case = Case::new();
    seed_two_groups(&mut case);

    let expected: Query<(), (Text,)> = sql("SELECT name FROM users ORDER BY name LIMIT 2 OFFSET 1");
    case.assert_same(expected, compile(|| dsl::limit_offset(2, 1)), ());
}

#[test]
fn fetch_first_rows_only() {
    let mut case = Case::new();
    seed_two_groups(&mut case);

    let expected: Query<(), (Text,)> =
        sql("SELECT name FROM users ORDER BY name OFFSET 1 ROWS FETCH FIRST 2 ROWS ONLY");
    case.assert_same(expected, compile(|| dsl::fetch_first_rows_only(2, 1)), ());
}

#[test]
fn fetch_first_with_ties() {
    let mut case = Case::new();
    seed_two_groups(&mut case);

    let expected: Query<(), (Nullable<Int8>,)> =
        sql("SELECT group_id FROM users ORDER BY group_id FETCH FIRST 1 ROWS WITH TIES");
    case.assert_same(expected, compile(|| dsl::fetch_first_with_ties(1)), ());
}

#[test]
fn aliases() {
    let mut case = Case::new();
    seed_two_groups(&mut case);

    let expected: Query<(), (Text, Text)> = sql("SELECT u.name AS user_name, g.name AS group_name
         FROM users AS u
         JOIN groups AS g ON g.id = u.group_id
         ORDER BY user_name");
    case.assert_same_named(expected, compile(dsl::aliases), ());
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

    let expected: Query<(Text,), (Text,)> =
        sql("SELECT name FROM tickets WHERE batch = $1 ORDER BY name FOR UPDATE");
    case.assert_same(expected, compile(dsl::for_update), (&batch,));
}

#[test]
fn for_share_alongside_other_share() {
    let mut case = Case::new();
    let (batch, locked) = seed_tickets(&mut case);
    let mut other = Case::new();
    other.exec(&format!(
        "SELECT 1 FROM tickets WHERE name = '{locked}' FOR SHARE"
    ));

    let expected: Query<(Text,), (Text,)> =
        sql("SELECT name FROM tickets WHERE batch = $1 ORDER BY name FOR SHARE");
    case.assert_same(expected, compile(dsl::for_share), (&batch,));
}

#[test]
fn for_update_skip_locked() {
    let mut case = Case::new();
    let (batch, locked) = seed_tickets(&mut case);
    let mut other = Case::new();
    other.exec(&format!(
        "SELECT 1 FROM tickets WHERE name = '{locked}' FOR UPDATE"
    ));

    let expected: Query<(Text,), (Text,)> =
        sql("SELECT name FROM tickets WHERE batch = $1 ORDER BY name FOR UPDATE SKIP LOCKED");
    case.assert_same(expected, compile(dsl::for_update_skip_locked), (&batch,));
}

#[test]
fn for_update_nowait_on_locked_row() {
    let mut case = Case::new();
    let (_, locked) = seed_tickets(&mut case);
    let mut other = Case::new();
    other.exec(&format!(
        "SELECT 1 FROM tickets WHERE name = '{locked}' FOR UPDATE"
    ));

    let expected: Query<(Text,), (Text,)> =
        sql("SELECT name FROM tickets WHERE name = $1 FOR UPDATE NOWAIT");
    case.assert_same_error(expected, compile(dsl::for_update_nowait), (&locked,));
}

#[test]
fn star() {
    let mut case = Case::new();
    seed_two_groups(&mut case);

    let expected: Query<(), (Int8, Text, Nullable<Text>, Nullable<Int8>)> =
        sql("SELECT * FROM users ORDER BY name");
    case.assert_same_named(expected, compile(dsl::star), ());
}

#[test]
fn table_star() {
    let mut case = Case::new();
    seed_two_groups(&mut case);

    let expected: Query<(), (Int8, Text, Nullable<Text>, Nullable<Int8>, Text)> = sql(
        "SELECT u.*, g.name AS group_name FROM users u JOIN groups g ON g.id = u.group_id ORDER BY u.name",
    );
    case.assert_same_named(expected, compile(dsl::table_star), ());
}

#[test]
fn without_from() {
    let mut case = Case::new();

    let expected: Query<(), (Int4, Text, Bool, Bool)> =
        sql("SELECT 1 + 1, 'a' || 'b', 10 > 3, CURRENT_DATE = CURRENT_DATE");
    case.assert_same(expected, compile(dsl::without_from), ());
}

#[test]
fn for_key_share_alongside_no_key_update() {
    let mut case = Case::new();
    let (batch, locked) = seed_tickets(&mut case);
    let mut other = Case::new();
    other.exec(&format!(
        "SELECT 1 FROM tickets WHERE name = '{locked}' FOR NO KEY UPDATE"
    ));

    let expected: Query<(Text,), (Text,)> =
        sql("SELECT name FROM tickets WHERE batch = $1 ORDER BY name FOR KEY SHARE");
    case.assert_same(expected, compile(dsl::for_key_share), (&batch,));
}

#[test]
fn for_no_key_update_skip_locked() {
    let mut case = Case::new();
    let (batch, locked) = seed_tickets(&mut case);
    let mut other = Case::new();
    other.exec(&format!(
        "SELECT 1 FROM tickets WHERE name = '{locked}' FOR NO KEY UPDATE"
    ));

    let expected: Query<(Text,), (Text,)> = sql(
        "SELECT name FROM tickets WHERE batch = $1 ORDER BY name FOR NO KEY UPDATE SKIP LOCKED",
    );
    case.assert_same(
        expected,
        compile(dsl::for_no_key_update_skip_locked),
        (&batch,),
    );
}

#[test]
fn for_update_of_table() {
    let mut case = Case::new();
    seed_two_groups(&mut case);

    let expected: Query<(), (Text, Text)> = sql(
        "SELECT u.name, g.name FROM users u JOIN groups g ON g.id = u.group_id ORDER BY u.name FOR UPDATE OF u",
    );
    case.assert_same(expected, compile(dsl::for_update_of_table), ());
}
