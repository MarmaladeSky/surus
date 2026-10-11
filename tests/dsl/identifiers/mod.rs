mod dsl;

use crate::support::{Case, compile, rnd, sql};
use surus::Query;
use surus::types::{Bool, Int4, Nullable, Numeric, Text};

#[test]
fn quoted_table_and_columns() {
    let mut case = Case::new();
    let user = rnd::text();
    let selected = rnd::int();

    let expected: Query<(Text, Int4), (Bool, Text, Int4, Nullable<Numeric>)> = sql(
        "INSERT INTO \"Order\" (\"user\", \"select\", \"line total\")
         VALUES ($1, $2, 12.5)
         RETURNING \"Id\" IS NOT NULL AS \"Has Id\", \"user\", \"select\", \"line total\"",
    );
    case.assert_same_named(
        expected,
        compile(dsl::quoted_table_and_columns),
        (&user, selected),
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

    let expected: Query<(), (Text, Int4)> = sql(
        "SELECT o.\"user\" AS \"User Name\", o.\"select\" AS \"Select\"
         FROM \"Order\" AS o
         ORDER BY \"User Name\"",
    );
    case.assert_same_named(expected, compile(dsl::quoted_aliases), ());
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

    let expected: Query<(), (Text, Text)> = sql("SELECT 'app', name FROM app.users
         UNION ALL
         SELECT 'public', public.users.name FROM public.users
         ORDER BY 1");
    case.assert_same(expected, compile(dsl::schema_qualified_tables), ());
}
