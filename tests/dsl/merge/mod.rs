mod dsl;

use crate::support::{Case, compile, rnd, sql};
use surus::Query;
use surus::types::{Int4, Nullable, Text};

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

    let expected: Query<(Text, Int4, Text, Int4), (Text, Text, Int4, Int4)> =
        sql("MERGE INTO products
         USING (VALUES ($1, $2::int4, 1), ($3, $4::int4, 2))
             AS src(name, price_cents, quantity)
         ON products.name = src.name
         WHEN MATCHED THEN UPDATE SET price_cents = src.price_cents
         WHEN NOT MATCHED THEN
             INSERT (name, price_cents, quantity) VALUES (src.name, src.price_cents, src.quantity)
         RETURNING merge_action(), products.name, products.price_cents, products.quantity");
    case.assert_same(
        expected,
        compile(dsl::upsert_from_values),
        (&existing, existing_price, &fresh, fresh_price),
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

    let expected: Query<(Text,), (Text, Text)> = sql("MERGE INTO products
         USING (VALUES ($1)) AS src(name)
         ON products.name = src.name
         WHEN MATCHED THEN DO NOTHING
         WHEN NOT MATCHED BY SOURCE THEN DELETE
         RETURNING merge_action(), products.name");
    case.assert_same(
        expected,
        compile(dsl::delete_not_matched_by_source),
        (&keep,),
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

    let expected: Query<(Text, Int4, Text), (Text, Nullable<Int4>, Int4, Text)> =
        sql("MERGE INTO products
         USING (VALUES ($1, $2::int4), ($3, $2::int4)) AS src(name, price_cents)
         ON products.name = src.name
         WHEN MATCHED THEN UPDATE SET price_cents = src.price_cents
         WHEN NOT MATCHED THEN INSERT (name, price_cents) VALUES (src.name, src.price_cents)
         RETURNING merge_action(), old.price_cents, new.price_cents, new.name");
    case.assert_same(
        expected,
        compile(dsl::returning_old_and_new),
        (&existing, price, &fresh),
    );
}

#[test]
fn conditional_when() {
    let mut case = Case::new();
    let sold_out = rnd::text();
    let in_stock = rnd::text();

    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity)
         VALUES ('{sold_out}', {}, 0), ('{in_stock}', {}, 2)",
        rnd::int(),
        rnd::int(),
    ));

    let expected: Query<(Text, Text), (Text, Text)> = sql("MERGE INTO products
         USING (VALUES ($1), ($2)) AS src(name)
         ON products.name = src.name
         WHEN MATCHED AND products.quantity = 0 THEN DELETE
         WHEN MATCHED THEN UPDATE SET quantity = products.quantity - 1
         RETURNING merge_action(), products.name");
    case.assert_same(
        expected,
        compile(dsl::conditional_when),
        (&sold_out, &in_stock),
    );
}
