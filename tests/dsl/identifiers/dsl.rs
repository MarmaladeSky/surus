use crate::schema::{app, order, users};
use surus::prelude::*;
use surus::types::{Bool, Int4, Nullable, Numeric, Text};
use surus::{Query, literal, query, value};

pub fn quoted_table_and_columns() -> Query<(Text, Int4), (Bool, Text, Int4, Nullable<Numeric>)> {
    query(|(user, selected)| {
        order
            .insert(|o| {
                o.user(user)
                    .r#select(selected)
                    .line_total(literal::<Numeric>("12.5"))
            })
            .returning(|o| {
                (
                    o.id.is_not_null().alias("Has Id"),
                    o.user,
                    o.r#select,
                    o.line_total,
                )
            })
    })
}

pub fn quoted_aliases() -> Query<(), (Text, Int4)> {
    order
        .select(|o| (o.user.alias("User Name"), o.r#select.alias("Select")))
        .order_by(|o| o.user)
        .into()
}

pub fn schema_qualified_tables() -> Query<(), (Text, Text)> {
    app::users
        .select(|u| (value("app"), u.name))
        .union_all(users.select(|u| (value("public"), u.name)))
        .order_by(|(schema, _)| schema)
        .into()
}
