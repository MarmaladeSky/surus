mod dsl;

use crate::support::{Case, rnd};
use postgres::types::Type;

#[test]
fn typed_parameter_list() {
    let mut case = Case::new();

    case.assert_param_types(&[Type::TEXT, Type::INT4, Type::INT8, Type::BOOL], || {
        dsl::typed_parameter_list("x", 1, 2, true)
    });
}

#[test]
fn many_parameters() {
    let mut case = Case::new();
    let names: Vec<String> = (0..10).map(|_| rnd::text()).collect();
    case.exec(&format!(
        "INSERT INTO users (name) VALUES {}, ('{}')",
        names
            .iter()
            .map(|n| format!("('{n}')"))
            .collect::<Vec<_>>()
            .join(", "),
        rnd::text()
    ));

    case.assert_same(
        &format!(
            "SELECT name FROM users WHERE name IN ({}) ORDER BY name",
            names
                .iter()
                .map(|n| format!("'{n}'"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        || dsl::many_parameters(&names),
    );
}

#[test]
fn parameter_reuse() {
    let mut case = Case::new();
    let shared = rnd::text();
    case.exec(&format!(
        "INSERT INTO users (name, email) VALUES ('{shared}', NULL), ('{}', '{shared}'), ('{}', NULL)",
        rnd::text(),
        rnd::text(),
    ));

    case.assert_same(
        &format!("SELECT name, email FROM users WHERE name = '{shared}' OR email = '{shared}' ORDER BY name"),
        || dsl::parameter_reuse(&shared),
    );
}
