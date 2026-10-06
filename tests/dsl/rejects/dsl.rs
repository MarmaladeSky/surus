use super::swap;
use crate::support::Query;
use schema::*;
use surus::*;

pub fn column_existence() -> Query {
    users
        .select(swap!(users.name, users.nme))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn column_scope() -> Query {
    let u = alias!(users, "u");
    u.select(swap!(u.name, users.name))
        .order_by(u.name)
        .compile()
        .into()
}

pub fn where_boolean() -> Query {
    users
        .select(users.name)
        .filter(swap!(users.email.is_null(), users.email))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn having_boolean() -> Query {
    users
        .select((users.group_id, count_star()))
        .group_by(users.group_id)
        .having(swap!(count_star().gt(1), count_star()))
        .order_by(users.group_id)
        .compile()
        .into()
}

pub fn join_on_boolean() -> Query {
    let (u, g) = (alias!(users, "u"), alias!(groups, "g"));
    let on = g.id.eq(u.group_id);
    u.join(g, swap!(on, g.id))
        .select((u.name, g.name))
        .order_by(u.name)
        .compile()
        .into()
}

pub fn update_where_boolean() -> Query {
    users
        .update(users.email.set(users.name))
        .filter(swap!(users.email.is_null(), users.email))
        .returning(users.name)
        .compile()
        .into()
}

pub fn delete_where_boolean() -> Query {
    users
        .delete()
        .filter(swap!(users.email.is_null(), users.email))
        .returning(users.name)
        .compile()
        .into()
}

pub fn case_when_boolean() -> Query {
    let email = when(swap!(users.email.is_null(), users.email), "none").else_(users.email);
    users.select(email).order_by(users.name).compile().into()
}

pub fn filter_boolean() -> Query {
    users
        .select(count_star().filter(swap!(users.email.is_not_null(), users.email)))
        .compile()
        .into()
}

pub fn comparison_types() -> Query {
    users
        .select(users.name)
        .filter(swap!(users.id.ne(users.group_id), users.id.ne(users.name)))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn operator_operands() -> Query {
    let t = type_samples;
    t.select(swap!(
        t.v_jsonb.contains(t.v_jsonb),
        t.v_jsonb.contains(t.v_int4)
    ))
    .compile()
    .into()
}

pub fn function_arguments() -> Query {
    users
        .select(lower(swap!(users.name, users.id)))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn cast_validity() -> Query {
    let id = swap!(users.id.cast::<Text>(), users.id.cast::<Uuid>());
    users.select(id.clone()).order_by(id).compile().into()
}

pub fn parameter_mapping(id: i64) -> Query {
    users
        .select(users.name)
        .filter(users.id.eq(swap!(id, id as u64)))
        .compile()
        .into()
}

pub fn insert_value_type() -> Query {
    products
        .insert((products.name, products.price_cents, products.quantity))
        .values(("p", swap!(10, "10"), 1))
        .returning(products.name)
        .compile()
        .into()
}

pub fn insert_not_null() -> Query {
    users
        .insert((users.name,))
        .values((swap!("x", null()),))
        .returning(users.name)
        .compile()
        .into()
}

pub fn insert_arity() -> Query {
    users
        .insert((users.name, users.email))
        .values(swap!(("x", null()), ("x",)))
        .returning(users.name)
        .compile()
        .into()
}

pub fn update_assignment_type() -> Query {
    products
        .update(
            products
                .quantity
                .set(swap!(products.quantity + 1, products.name)),
        )
        .returning((products.name, products.quantity))
        .compile()
        .into()
}

pub fn nullability_propagation() -> Query {
    let names = users.select(swap!(users.name.concat("!"), users.email.concat("!")));
    users
        .insert((users.name,))
        .select(names)
        .returning(users.name)
        .compile()
        .into()
}

pub fn result_row_type() -> Query {
    let rows: Compiled<(Int8, Text)> = swap!(
        users.select((users.id, users.name)),
        users.select((users.id, users.email))
    )
    .order_by(users.id)
    .compile();
    rows.into()
}

pub fn returning_type() -> Query {
    let rows: Compiled<(Bool, Text)> = users
        .insert((users.name,))
        .values(("x",))
        .returning(swap!(
            (users.id.is_not_null(), users.name),
            (users.id, users.name)
        ))
        .compile();
    rows.into()
}

pub fn cte_column_type() -> Query {
    let t = cte("t", users.select((users.id, users.name)));
    let (id, name) = t.columns();
    t.select(name)
        .filter(swap!(id.gt(0), name.gt(0)))
        .order_by(name)
        .compile()
        .into()
}

pub fn union_column_types() -> Query {
    users
        .select((users.id, users.name))
        .union(swap!(
            groups.select((groups.id, groups.name)),
            groups.select((groups.name, groups.id))
        ))
        .order_by(|(_, name)| name)
        .compile()
        .into()
}

pub fn union_arity() -> Query {
    let (id, name) = (groups.id, groups.name);
    users
        .select((users.id, users.name))
        .union(groups.select(swap!((id, name), id)))
        .order_by(|(_, name)| name)
        .compile()
        .into()
}

pub fn scalar_subquery_shape() -> Query {
    let (u, g) = (alias!(users, "u"), alias!(groups, "g"));
    let name = g.name;
    let group_name = g
        .select(swap!(name, (name, name)))
        .filter(g.id.eq(u.group_id));
    u.select((u.name, group_name))
        .order_by(u.name)
        .compile()
        .into()
}

pub fn in_subquery_type() -> Query {
    users
        .select(users.name)
        .filter(
            users
                .group_id
                .in_(groups.select(swap!(groups.id, groups.name))),
        )
        .order_by(users.name)
        .compile()
        .into()
}
