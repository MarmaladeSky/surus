use crate::support::Query;
use schema::*;
use surus::*;

pub fn case_searched() -> Query {
    let status = when(users.email.is_null(), "no email")
        .when(users.group_id.is_null(), "no group")
        .else_("complete");
    users
        .select((users.name, status))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn case_simple() -> Query {
    let stock = case(products.quantity)
        .when(0, "out")
        .when(1, "last")
        .else_("in stock");
    products
        .select((products.name, stock))
        .order_by(products.name)
        .compile()
        .into()
}

pub fn coalesce() -> Query {
    users
        .select((users.name, surus::coalesce(users.email, users.name)))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn nullif() -> Query {
    products
        .select((products.name, surus::nullif(products.quantity, 0)))
        .order_by(products.name)
        .compile()
        .into()
}

pub fn greatest_least() -> Query {
    let stock_value = products.quantity * 100;
    products
        .select((
            products.name,
            greatest(products.price_cents, stock_value.clone()),
            least(products.price_cents, stock_value),
        ))
        .order_by(products.name)
        .compile()
        .into()
}

pub fn case_into_arithmetic() -> Query {
    let price = when(products.quantity.gt(0), products.price_cents).else_(0);
    products
        .select((products.name, price + 1))
        .order_by(products.name)
        .compile()
        .into()
}
