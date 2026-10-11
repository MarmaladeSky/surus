mod dsl;

use crate::support::{Case, compile, rnd, sql};
use surus::Query;
use surus::types::{Bool, Int4, Int8, Nullable, Text};

const DEPTH: usize = 30;

#[test]
fn users_with_group() {
    let mut case = Case::new();
    let group = rnd::text();
    let member = rnd::text();
    let loner = rnd::text();

    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{group}');
         INSERT INTO users (name, group_id)
             VALUES ('{member}', (SELECT id FROM groups WHERE name = '{group}'));
         INSERT INTO users (name) VALUES ('{loner}');"
    ));

    let expected: Query<(), (Text, Text)> = sql("SELECT users.name, groups.name
         FROM users
         JOIN groups ON groups.id = users.group_id
         ORDER BY users.name");
    case.assert_same(expected, compile(dsl::users_with_group), ());
}

#[test]
fn group_ancestor_30_levels() {
    let mut case = Case::new();
    let chain: Vec<String> = (0..=DEPTH).map(|_| rnd::text()).collect();

    let mut init = format!("INSERT INTO groups (name) VALUES ('{}');\n", chain[0]);
    for pair in chain.windows(2) {
        init += &format!(
            "INSERT INTO groups (name, parent_id)
                 VALUES ('{}', (SELECT id FROM groups WHERE name = '{}'));\n",
            pair[1], pair[0]
        );
    }
    init += &format!("INSERT INTO groups (name) VALUES ('{}');\n", rnd::text());
    case.exec(&init);

    let joins: String = (1..=DEPTH)
        .map(|i| format!("JOIN groups g{i} ON g{i}.id = g{}.parent_id\n", i - 1))
        .collect();
    let plain = format!("SELECT g0.name, g{DEPTH}.name FROM groups g0\n{joins}");

    let expected: Query<(), (Text, Text)> = sql(&plain);
    case.assert_same(expected, compile(dsl::group_ancestor_30_levels), ());
}

type Logins = (
    Text,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
    Int4,
);

#[test]
fn logins_30_day_pivot() {
    let mut case = Case::new();

    let mut init = String::new();
    for _ in 0..2 {
        let name = rnd::text();
        init += &format!("INSERT INTO users (name) VALUES ('{name}');\n");
        for day in 1..=DEPTH {
            init += &format!(
                "INSERT INTO logins_day_{day:02} (user_id, logins)
                     VALUES ((SELECT id FROM users WHERE name = '{name}'), {});\n",
                rnd::u64() % 100
            );
        }
    }
    case.exec(&init);

    let columns: String = (1..=DEPTH).map(|d| format!(", d{d:02}.logins")).collect();
    let joins: String = (1..=DEPTH)
        .map(|d| format!("JOIN logins_day_{d:02} d{d:02} ON d{d:02}.user_id = users.id\n"))
        .collect();
    let plain = format!("SELECT users.name{columns} FROM users\n{joins}ORDER BY users.name");

    let expected: Query<(), Logins> = sql(&plain);
    case.assert_same(expected, compile(dsl::logins_30_day_pivot), ());
}

fn seed_groups(case: &mut Case) {
    let big = rnd::text();
    let small = rnd::text();
    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{big}'), ('{small}'), ('{}');
         INSERT INTO users (name, group_id)
             SELECT '{}', id FROM groups WHERE name = '{big}'
             UNION ALL SELECT '{}', id FROM groups WHERE name = '{big}'
             UNION ALL SELECT '{}', id FROM groups WHERE name = '{small}'
             UNION ALL SELECT '{}', NULL;",
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));
}

fn seed_extensions(case: &mut Case) {
    let full = rnd::text();
    let partial = rnd::text();
    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('{full}'), ('{partial}');
         INSERT INTO user_profiles (user_id, bio)
             SELECT id, '{}' FROM users WHERE name = '{full}'
             UNION ALL SELECT id, '{}' FROM users WHERE name = '{partial}';
         INSERT INTO user_settings (user_id, theme)
             SELECT id, '{}' FROM users WHERE name = '{full}';",
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));
}

#[test]
fn left_join() {
    let mut case = Case::new();
    seed_groups(&mut case);

    let expected: Query<(), (Text, Nullable<Text>)> = sql("SELECT users.name, groups.name
         FROM users
         LEFT JOIN groups ON groups.id = users.group_id
         ORDER BY users.name");
    case.assert_same(expected, compile(dsl::left_join), ());
}

#[test]
fn left_join_null_propagation() {
    let mut case = Case::new();
    seed_groups(&mut case);

    let expected: Query<(), (Text, Text, Bool)> = sql(
        "SELECT users.name, coalesce(groups.name, '<none>'), groups.id IS NULL
         FROM users
         LEFT JOIN groups ON groups.id = users.group_id
         ORDER BY users.name",
    );
    case.assert_same(expected, compile(dsl::left_join_null_propagation), ());
}

#[test]
fn right_join() {
    let mut case = Case::new();
    seed_groups(&mut case);

    let expected: Query<(), (Nullable<Text>, Text)> = sql("SELECT users.name, groups.name
         FROM users
         RIGHT JOIN groups ON groups.id = users.group_id
         ORDER BY groups.name, users.name");
    case.assert_same(expected, compile(dsl::right_join), ());
}

#[test]
fn full_join() {
    let mut case = Case::new();
    seed_groups(&mut case);

    let expected: Query<(), (Nullable<Text>, Nullable<Text>)> =
        sql("SELECT users.name, groups.name
         FROM users
         FULL JOIN groups ON groups.id = users.group_id
         ORDER BY groups.name, users.name");
    case.assert_same(expected, compile(dsl::full_join), ());
}

#[test]
fn cross_join() {
    let mut case = Case::new();
    seed_groups(&mut case);

    let expected: Query<(), (Text, Text)> =
        sql("SELECT users.name, groups.name FROM users CROSS JOIN groups ORDER BY 1, 2");
    case.assert_same(expected, compile(dsl::cross_join), ());
}

#[test]
fn natural_join() {
    let mut case = Case::new();
    seed_extensions(&mut case);

    let expected: Query<(), (Int8, Text, Text)> = sql("SELECT user_id, bio, theme
         FROM user_profiles NATURAL JOIN user_settings
         ORDER BY user_id");
    case.assert_same(expected, compile(dsl::natural_join), ());
}

#[test]
fn join_using() {
    let mut case = Case::new();
    seed_extensions(&mut case);

    let expected: Query<(), (Int8, Text, Nullable<Text>)> = sql("SELECT user_id, bio, theme
         FROM user_profiles LEFT JOIN user_settings USING (user_id)
         ORDER BY user_id");
    case.assert_same(expected, compile(dsl::join_using), ());
}

#[test]
fn lateral_first_user_per_group() {
    let mut case = Case::new();
    seed_groups(&mut case);

    let expected: Query<(), (Text, Nullable<Text>)> = sql("SELECT g.name, u.name
         FROM groups g
         LEFT JOIN LATERAL (
             SELECT name FROM users WHERE users.group_id = g.id ORDER BY name LIMIT 1
         ) u ON true
         ORDER BY g.name");
    case.assert_same(expected, compile(dsl::lateral_first_user_per_group), ());
}

#[test]
fn subquery_in_from() {
    let mut case = Case::new();
    seed_groups(&mut case);

    let expected: Query<(), (Text, Int8)> = sql("SELECT g.name, c.n
         FROM groups g
         JOIN (SELECT group_id, count(*) AS n FROM users GROUP BY group_id) c
             ON c.group_id = g.id
         ORDER BY g.name");
    case.assert_same(expected, compile(dsl::subquery_in_from), ());
}

#[test]
fn tablesample_repeatable() {
    let mut case = Case::new();
    let names: Vec<String> = (0..20).map(|_| format!("('{}')", rnd::text())).collect();
    case.exec(&format!(
        "INSERT INTO users (name) VALUES {}",
        names.join(", ")
    ));

    let expected: Query<(), (Text,)> =
        sql("SELECT name FROM users TABLESAMPLE BERNOULLI (50) REPEATABLE (42) ORDER BY name");
    case.assert_same(expected, compile(dsl::tablesample_repeatable), ());
}

#[test]
fn with_ordinality() {
    let mut case = Case::new();
    seed_groups(&mut case);

    let expected: Query<(), (Text, Text, Int8)> = sql("SELECT u.name, w.part, w.ord
         FROM users u
         CROSS JOIN LATERAL unnest(string_to_array(u.name, '0')) WITH ORDINALITY AS w(part, ord)
         ORDER BY u.name, w.ord");
    case.assert_same(expected, compile(dsl::with_ordinality), ());
}

#[test]
fn set_returning_function_in_from() {
    let mut case = Case::new();
    seed_groups(&mut case);

    let expected: Query<(), (Text, Int4)> = sql(
        "SELECT users.name, s.day FROM users CROSS JOIN generate_series(1, 3) AS s(day) ORDER BY 1, 2",
    );
    case.assert_same(
        expected,
        compile(|| dsl::set_returning_function_in_from(1, 3)),
        (),
    );
}

#[test]
fn rows_from_multiple_functions() {
    let mut case = Case::new();
    seed_groups(&mut case);

    let expected: Query<(), (Text, Nullable<Int4>, Nullable<Text>, Int8)> =
        sql("SELECT u.name, r.n, r.part, r.ord
         FROM users u
         CROSS JOIN LATERAL ROWS FROM (generate_series(1, 2), unnest(string_to_array(u.name, '0')))
             WITH ORDINALITY AS r(n, part, ord)
         ORDER BY u.name, r.ord");
    case.assert_same(expected, compile(dsl::rows_from_multiple_functions), ());
}

fn unique_id() -> i64 {
    900_000_000 + (rnd::u64() % 100_000_000) as i64
}

#[test]
fn function_columns_in_join_on() {
    let mut case = Case::new();
    let base = unique_id();
    case.exec(&format!(
        "INSERT INTO users (id, name) VALUES ({base}, '{}'), ({}, '{}'), ({}, '{}')",
        rnd::text(),
        base + 1,
        rnd::text(),
        base + 5,
        rnd::text(),
    ));

    let expected: Query<(Int8, Int8), (Text, Int8)> = sql(
        "SELECT u.name, s.n FROM generate_series($1::int8, $2::int8) AS s(n)
         JOIN users u ON u.id = s.n
         ORDER BY s.n",
    );
    case.assert_same(
        expected,
        compile(dsl::function_columns_in_join_on),
        (base, base + 2),
    );
}

#[test]
fn ordinality_column_in_where() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('a0{}0b'), ('c0{}')",
        rnd::text(),
        rnd::text(),
    ));

    let expected: Query<(), (Text, Text)> = sql("SELECT u.name, w.part FROM users u
         CROSS JOIN LATERAL unnest(string_to_array(u.name, '0')) WITH ORDINALITY AS w(part, ord)
         WHERE w.ord > 1
         ORDER BY u.name, w.ord");
    case.assert_same(expected, compile(dsl::ordinality_column_in_where), ());
}

#[test]
fn values_in_from_joined() {
    let mut case = Case::new();
    let first = unique_id();
    let second = unique_id();
    case.exec(&format!(
        "INSERT INTO users (id, name) VALUES ({first}, '{}'), ({second}, '{}'), ({}, '{}')",
        rnd::text(),
        rnd::text(),
        unique_id(),
        rnd::text(),
    ));

    let expected: Query<(Int8, Int8), (Text, Text)> = sql("SELECT u.name, v.label FROM users u
         JOIN (VALUES ($1::int8, 'a'), ($2::int8, 'b')) AS v(id, label) ON v.id = u.id
         ORDER BY v.label");
    case.assert_same(
        expected,
        compile(dsl::values_in_from_joined),
        (first, second),
    );
}

#[test]
fn self_join_aliases() {
    let mut case = Case::new();
    seed_groups(&mut case);

    let expected: Query<(), (Text, Text)> = sql("SELECT a.name, b.name FROM users a
         JOIN users b ON a.group_id = b.group_id AND a.id < b.id
         ORDER BY a.name, b.name");
    case.assert_same(expected, compile(dsl::self_join_aliases), ());
}

#[test]
fn lateral_columns_in_projection_expr() {
    let mut case = Case::new();
    seed_groups(&mut case);

    let expected: Query<(), (Text, Int8)> = sql("SELECT g.name, l.cnt + 1 FROM groups g
         CROSS JOIN LATERAL (SELECT count(*) AS cnt FROM users u WHERE u.group_id = g.id) l
         ORDER BY g.name");
    case.assert_same(
        expected,
        compile(dsl::lateral_columns_in_projection_expr),
        (),
    );
}

#[test]
fn subquery_columns_in_where() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity) VALUES ('{}', 5, 1), ('{}', 7, 3), ('{}', 9, 4)",
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    let expected: Query<(), (Text, Int4)> = sql(
        "SELECT s.name, s.total FROM (SELECT name, price_cents * quantity AS total FROM products) s
         WHERE s.total > 10
         ORDER BY s.name",
    );
    case.assert_same(expected, compile(dsl::subquery_columns_in_where), ());
}
