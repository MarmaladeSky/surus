use crate::support::Query;
use schema::*;
use surus::*;

pub fn quoted_table_and_columns(user: &str, selected: i32) -> Query {
    order
        .insert((order.user, order.select, order.line_total))
        .values((user, selected, Decimal::new(125, 1)))
        .returning((
            order.id.is_not_null().as_("Has Id"),
            order.user,
            order.select,
            order.line_total,
        ))
        .compile()
        .into()
}

pub fn quoted_aliases() -> Query {
    let o = alias!(order, "o");
    o.select((o.user.as_("User Name"), o.select.as_("Select")))
        .order_by(o.user)
        .compile()
        .into()
}

pub fn schema_qualified_tables() -> Query {
    app::users
        .select(("app", app::users.name))
        .union_all(users.select(("public", users.name)))
        .order_by(|(schema, _)| schema)
        .compile()
        .into()
}
