mod dsl;

use crate::support::{Case, rnd};

#[test]
fn upsert_from_values() {
    let mut case = Case::new();
    let existing = rnd::text();
    let fresh = rnd::text();
    let existing_price = rnd::int();
    let fresh_price = rnd::int();

    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity) VALUES ('{existing}', {}, 1)",
        rnd::int()
    ));

    case.assert_same(
        &format!(
            "MERGE INTO products
             USING (VALUES ('{existing}', {existing_price}, 1), ('{fresh}', {fresh_price}, 2))
                 AS src(name, price_cents, quantity)
             ON products.name = src.name
             WHEN MATCHED THEN UPDATE SET price_cents = src.price_cents
             WHEN NOT MATCHED THEN
                 INSERT (name, price_cents, quantity) VALUES (src.name, src.price_cents, src.quantity)
             RETURNING merge_action(), products.name, products.price_cents, products.quantity"
        ),
        || dsl::upsert_from_values(&existing, existing_price, &fresh, fresh_price),
    );
}

#[test]
fn delete_not_matched_by_source() {
    let mut case = Case::new();
    let keep = rnd::text();

    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity)
         VALUES ('{keep}', {}, 1), ('{}', {}, 1), ('{}', {}, 1)",
        rnd::int(),
        rnd::text(),
        rnd::int(),
        rnd::text(),
        rnd::int(),
    ));

    case.assert_same(
        &format!(
            "MERGE INTO products
             USING (VALUES ('{keep}')) AS src(name)
             ON products.name = src.name
             WHEN MATCHED THEN DO NOTHING
             WHEN NOT MATCHED BY SOURCE THEN DELETE
             RETURNING merge_action(), products.name"
        ),
        || dsl::delete_not_matched_by_source(&keep),
    );
}

#[test]
fn returning_old_and_new() {
    let mut case = Case::new();
    let existing = rnd::text();
    let fresh = rnd::text();
    let price = rnd::int();
    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity) VALUES ('{existing}', {}, 1)",
        rnd::int()
    ));

    case.assert_same(
        &format!(
            "MERGE INTO products
             USING (VALUES ('{existing}', {price}), ('{fresh}', {price})) AS src(name, price_cents)
             ON products.name = src.name
             WHEN MATCHED THEN UPDATE SET price_cents = src.price_cents
             WHEN NOT MATCHED THEN INSERT (name, price_cents) VALUES (src.name, src.price_cents)
             RETURNING merge_action(), old.price_cents, new.price_cents, new.name"
        ),
        || dsl::returning_old_and_new(&existing, &fresh, price),
    );
}
