use crate::support::Query;
use schema::*;
use surus::*;

pub fn user_returning(name: &str, email: &str) -> Query {
    users
        .insert((users.name, users.email))
        .values((name, email))
        .returning((users.name, users.email))
        .compile()
        .into()
}

pub fn product(name: &str, price_cents: i32, quantity: i32) -> Query {
    products
        .insert((products.name, products.price_cents, products.quantity))
        .values((name, price_cents, quantity))
        .returning((
            products.name,
            products.price_cents,
            products.quantity,
            products.total_cents,
        ))
        .compile()
        .into()
}

pub fn insert_select(pattern: &str) -> Query {
    let names = users
        .select(users.name)
        .filter(users.name.like(pattern))
        .order_by(users.name);
    groups
        .insert((groups.name,))
        .select(names)
        .returning(groups.name)
        .compile()
        .into()
}

pub fn multi_row_values(first: &str, email: &str, second: &str) -> Query {
    users
        .insert((users.name, users.email))
        .values((first, email))
        .values((second, null()))
        .returning((users.name, users.email))
        .compile()
        .into()
}

pub fn default_value(name: &str, price_cents: i32) -> Query {
    products
        .insert((products.name, products.price_cents, products.quantity))
        .values((name, price_cents, default()))
        .returning((products.name, products.price_cents, products.quantity))
        .compile()
        .into()
}

pub fn overriding_system_value(id: i64, name: &str) -> Query {
    products
        .insert((products.id, products.name, products.price_cents))
        .overriding_system_value()
        .values((id, name, 1))
        .returning((products.id, products.name))
        .compile()
        .into()
}

pub fn on_conflict_do_nothing(taken: &str, free: &str) -> Query {
    let candidates = users
        .select((users.id, "light"))
        .filter(users.name.in_([taken, free]));
    user_settings
        .insert((user_settings.user_id, user_settings.theme))
        .select(candidates)
        .on_conflict(user_settings.user_id)
        .do_nothing()
        .returning((user_settings.user_id, user_settings.theme))
        .compile()
        .into()
}

pub fn on_conflict_do_update(name: &str, theme: &str) -> Query {
    let excluded = user_settings.excluded();
    user_settings
        .insert((user_settings.user_id, user_settings.theme))
        .select(users.select((users.id, theme)).filter(users.name.eq(name)))
        .on_conflict(user_settings.user_id)
        .do_update(user_settings.theme.set(excluded.theme.concat("!")))
        .filter(user_settings.theme.ne(excluded.theme))
        .returning((user_settings.user_id, user_settings.theme))
        .compile()
        .into()
}

pub fn default_values() -> Query {
    type_samples
        .insert_default_values()
        .returning((type_samples.v_int4.is_null(), type_samples.v_text.is_null()))
        .compile()
        .into()
}

pub fn virtual_generated_column(name: &str, price_cents: i32) -> Query {
    products
        .insert((products.name, products.price_cents))
        .values((name, price_cents))
        .returning((
            products.price_cents,
            products.price_with_tax,
            products.total_cents,
        ))
        .compile()
        .into()
}

pub fn on_conflict_partial_index(name: &str, email: &str) -> Query {
    let excluded = users.excluded();
    users
        .insert((users.name, users.email))
        .values((name, email))
        .on_conflict(users.email)
        .filter(users.email.is_not_null())
        .do_update(users.name.set(excluded.name))
        .returning((users.name, users.email))
        .compile()
        .into()
}

pub fn returning_expression(name: &str, price_cents: i32, quantity: i32) -> Query {
    products
        .insert((products.name, products.price_cents, products.quantity))
        .values((name, price_cents, quantity))
        .returning((products.price_cents * products.quantity).as_("total"))
        .compile()
        .into()
}

pub fn insert_select_expression() -> Query {
    let copies = groups.select(groups.name.concat("-copy"));
    groups
        .insert((groups.name,))
        .select(copies)
        .returning(groups.name)
        .compile()
        .into()
}

pub fn insert_bound(name: &str, email: Option<&str>) -> Query {
    users
        .insert((users.name, users.email))
        .values((name, email))
        .returning((users.name, users.email))
        .compile()
        .into()
}
