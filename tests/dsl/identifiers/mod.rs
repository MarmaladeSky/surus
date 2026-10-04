mod dsl;

use crate::support::{Case, rnd};

#[test]
fn quoted_table_and_columns() {
    let mut case = Case::new();
    let user = rnd::text();
    let selected = rnd::int();

    case.assert_same_named(
        &format!(
            "INSERT INTO \"Order\" (\"user\", \"select\", \"line total\")
             VALUES ('{user}', {selected}, 12.5)
             RETURNING \"Id\" IS NOT NULL AS \"Has Id\", \"user\", \"select\", \"line total\""
        ),
        || dsl::quoted_table_and_columns(&user, selected),
    );
}

#[test]
fn quoted_aliases() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO \"Order\" (\"user\", \"select\") VALUES ('{}', {}), ('{}', {})",
        rnd::text(),
        rnd::int(),
        rnd::text(),
        rnd::int(),
    ));

    case.assert_same_named(
        "SELECT o.\"user\" AS \"User Name\", o.\"select\" AS \"Select\"
         FROM \"Order\" AS o
         ORDER BY \"User Name\"",
        dsl::quoted_aliases,
    );
}

#[test]
fn schema_qualified_tables() {
    let mut case = Case::new();
    let in_app = rnd::text();
    let in_public = rnd::text();
    case.exec(&format!(
        "INSERT INTO app.users (name) VALUES ('{in_app}');
         INSERT INTO public.users (name) VALUES ('{in_public}');"
    ));

    case.assert_same(
        "SELECT 'app', name FROM app.users
         UNION ALL
         SELECT 'public', public.users.name FROM public.users
         ORDER BY 1",
        dsl::schema_qualified_tables,
    );
}
