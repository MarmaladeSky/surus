use crate::support::Query;
use schema::*;
use surus::*;

pub fn set_email(name: &str, email: &str) -> Query {
    users
        .update(users.email.set(email))
        .filter(users.name.eq(name))
        .returning((users.name, users.email))
        .compile()
        .into()
}

pub fn update_from() -> Query {
    users
        .update(users.email.set(groups.name.concat("@example.com")))
        .from(groups)
        .filter(groups.id.eq(users.group_id))
        .returning((users.name, users.email))
        .compile()
        .into()
}

pub fn set_row_from_subquery(user: &str, group: &str) -> Query {
    let g = alias!(groups, "g");
    let group_row = g
        .select((g.name.concat("@example.com"), g.id))
        .filter(g.name.eq(group));
    users
        .update((users.email, users.group_id).set(group_row))
        .filter(users.name.eq(user))
        .returning((users.name, users.email, users.group_id.is_not_null()))
        .compile()
        .into()
}

pub fn set_row_constructor(user: &str, renamed: &str, email: &str) -> Query {
    users
        .update((users.name, users.email).set(row((renamed, email))))
        .filter(users.name.eq(user))
        .returning((users.name, users.email))
        .compile()
        .into()
}

pub fn returning_star(name: &str) -> Query {
    products
        .update(products.quantity.set(products.quantity + 1))
        .filter(products.name.eq(name))
        .returning(products.star())
        .compile()
        .into()
}

pub fn returning_old_and_new(name: &str, delta: i32) -> Query {
    let (old, new) = (products.old(), products.new());
    products
        .update(products.quantity.set(products.quantity + delta))
        .filter(products.name.eq(name))
        .returning((
            old.quantity,
            new.quantity,
            new.total_cents - old.total_cents,
        ))
        .compile()
        .into()
}

pub fn update_set_expression() -> Query {
    products
        .update(products.quantity.set(products.quantity * 2 + 1))
        .filter(products.price_cents.gt(0))
        .returning((products.name, products.quantity))
        .compile()
        .into()
}

pub fn update_from_join_predicate() -> Query {
    let u = alias!(users, "u");
    let g = alias!(groups, "g");
    u.update(u.email.set(g.name.concat("@x")))
        .from(g)
        .filter(g.id.eq(u.group_id).and(u.email.is_null()))
        .returning((u.name, u.email))
        .compile()
        .into()
}

pub fn update_bound(id: i64, name: &str) -> Query {
    users
        .update(users.name.set(name))
        .filter(users.id.eq(id))
        .returning((users.id, users.name))
        .compile()
        .into()
}
