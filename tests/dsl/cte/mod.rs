mod dsl;

use crate::support::{Case, rnd};

#[test]
fn named_users() {
    let mut case = Case::new();
    let wanted = rnd::text();
    let other = rnd::text();

    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('{wanted}'), ('{other}')"
    ));

    case.assert_same(
        &format!(
            "WITH named AS (SELECT id, name FROM users WHERE name = '{wanted}')
             SELECT name FROM named"
        ),
        || dsl::named_users(&wanted),
    );
}

fn seed_chain(case: &mut Case) -> (String, String) {
    let root = rnd::text();
    let middle = rnd::text();
    let leaf = rnd::text();
    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{root}'), ('{}');
         INSERT INTO groups (name, parent_id) VALUES ('{middle}', (SELECT id FROM groups WHERE name = '{root}'));
         INSERT INTO groups (name, parent_id) VALUES ('{leaf}', (SELECT id FROM groups WHERE name = '{middle}'));",
        rnd::text()
    ));
    (root, leaf)
}

#[test]
fn recursive_ancestors() {
    let mut case = Case::new();
    let (_, leaf) = seed_chain(&mut case);

    case.assert_same(
        &format!(
            "WITH RECURSIVE ancestors AS (
                 SELECT id, name, parent_id, 0 AS depth FROM groups WHERE name = '{leaf}'
                 UNION ALL
                 SELECT g.id, g.name, g.parent_id, a.depth + 1
                 FROM groups g JOIN ancestors a ON g.id = a.parent_id
             )
             SELECT name, depth FROM ancestors ORDER BY depth"
        ),
        || dsl::recursive_ancestors(&leaf),
    );
}

#[test]
fn recursive_descendants_with_path() {
    let mut case = Case::new();
    let (root, _) = seed_chain(&mut case);

    case.assert_same(
        &format!(
            "WITH RECURSIVE tree(id, path) AS (
                 SELECT id, ARRAY[name] FROM groups WHERE name = '{root}'
                 UNION ALL
                 SELECT g.id, t.path || g.name FROM groups g JOIN tree t ON g.parent_id = t.id
             )
             SELECT path FROM tree ORDER BY path"
        ),
        || dsl::recursive_descendants_with_path(&root),
    );
}

#[test]
fn data_modifying() {
    let mut case = Case::new();
    let group = rnd::text();
    let batch = rnd::text();
    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{group}');
         INSERT INTO users (name, group_id)
             SELECT '{}', id FROM groups WHERE name = '{group}'
             UNION ALL SELECT '{}', id FROM groups WHERE name = '{group}'
             UNION ALL SELECT '{}', NULL;",
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    case.assert_same(
        &format!(
            "WITH archived AS (
                 DELETE FROM users WHERE group_id = (SELECT id FROM groups WHERE name = '{group}')
                 RETURNING name
             )
             INSERT INTO tickets (batch, name) SELECT '{batch}', name FROM archived ORDER BY name
             RETURNING batch, name"
        ),
        || dsl::data_modifying(&group, &batch),
    );
}

#[test]
fn chained_materialized() {
    let mut case = Case::new();
    seed_chain(&mut case);

    case.assert_same(
        "WITH roots AS MATERIALIZED (SELECT id, name FROM groups WHERE parent_id IS NULL),
              children AS NOT MATERIALIZED (
                  SELECT g.name, r.name AS root FROM groups g JOIN roots r ON g.parent_id = r.id
              )
         SELECT name, root FROM children ORDER BY name",
        dsl::chained_materialized,
    );
}

#[test]
fn cte_column_expressions() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity) VALUES ('{}', 5, 1), ('{}', 7, 3), ('{}', 9, 4)",
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    case.assert_same(
        "WITH t AS (SELECT name, price_cents * quantity AS total FROM products)
         SELECT name, total + 1 FROM t WHERE total > 10 ORDER BY name",
        dsl::cte_column_expressions,
    );
}

#[test]
fn cte_joined_to_table() {
    let mut case = Case::new();
    let big = rnd::text();
    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{big}'), ('{}');
         INSERT INTO users (name, group_id)
             SELECT '{}', id FROM groups WHERE name = '{big}'
             UNION ALL SELECT '{}', id FROM groups WHERE name = '{big}'
             UNION ALL SELECT '{}', id FROM groups WHERE name <> '{big}';",
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    case.assert_same(
        "WITH big AS (SELECT group_id FROM users GROUP BY group_id HAVING count(*) > 1)
         SELECT g.name FROM groups g JOIN big b ON b.group_id = g.id ORDER BY g.name",
        dsl::cte_joined_to_table,
    );
}

#[test]
fn cte_referenced_twice() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('{}'), ('{}')",
        rnd::text(),
        rnd::text(),
    ));

    case.assert_same(
        "WITH t AS (SELECT id, name FROM users)
         SELECT a.name, b.name FROM t a JOIN t b ON a.id = b.id ORDER BY a.name",
        dsl::cte_referenced_twice,
    );
}

#[test]
fn cte_in_subquery() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO users (name, email) VALUES ('{}', '{}'), ('{}', NULL)",
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    case.assert_same(
        "WITH t AS (SELECT id FROM users WHERE email IS NOT NULL)
         SELECT name FROM users WHERE id IN (SELECT id FROM t) ORDER BY name",
        dsl::cte_in_subquery,
    );
}

#[test]
fn recursive_depth_expression() {
    let mut case = Case::new();
    seed_chain(&mut case);

    case.assert_same(
        "WITH RECURSIVE tree(id, name, depth) AS (
             SELECT id, name, 0 FROM groups WHERE parent_id IS NULL
             UNION ALL
             SELECT g.id, g.name, t.depth + 1 FROM groups g JOIN tree t ON g.parent_id = t.id
         )
         SELECT name, depth * 10 FROM tree ORDER BY depth, name",
        dsl::recursive_depth_expression,
    );
}

#[test]
fn modifying_cte_returning_used() {
    let mut case = Case::new();
    let name = rnd::text();

    case.assert_same(
        &format!(
            "WITH ins AS (INSERT INTO groups (name) VALUES ('{name}') RETURNING id, name)
             SELECT name || '!' FROM ins"
        ),
        || dsl::modifying_cte_returning_used(&name),
    );
}
