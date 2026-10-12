use super::swap;
use crate::schema::{groups, products, type_samples, users};
use surus::prelude::*;
use surus::types::{Bool, Int4, Int8, Nullable, Text};
use surus::{Query, count_star, cte, from, lower, null, query, when};

pub fn column_existence() -> Query<(), (Text,)> {
    users
        .select(|u| swap!(u.name, u.nickname))
        .order_by(|u| u.name)
        .into()
}

pub fn column_scope() -> Query<(), (Text,)> {
    users
        .select(|u| swap!(u.name, groups.name))
        .order_by(|u| u.name)
        .into()
}

pub fn where_boolean() -> Query<(), (Text,)> {
    users
        .filter(|u| swap!(u.email.is_null(), u.email))
        .select(|u| u.name)
        .order_by(|u| u.name)
        .into()
}

pub fn having_boolean() -> Query<(), (Nullable<Int8>, Int8)> {
    users
        .group_by(|u| u.group_id)
        .having(|_| swap!(count_star().gt(1), count_star()))
        .select(|u| (u.group_id, count_star()))
        .order_by(|u| u.group_id)
        .into()
}

pub fn join_on_boolean() -> Query<(), (Text, Text)> {
    users
        .join(groups, |(u, g)| {
            swap!(g.id.eq(u.group_id), (g.id + u.group_id))
        })
        .select(|(u, g)| (u.name, g.name))
        .order_by(|(u, _)| u.name)
        .into()
}

pub fn update_where_boolean() -> Query<(), (Text,)> {
    users
        .filter(|u| swap!(u.email.is_null(), u.email))
        .update(|u| u.email.set(u.name))
        .returning(|u| u.name)
        .into()
}

pub fn delete_where_boolean() -> Query<(), (Text,)> {
    users
        .filter(|u| swap!(u.email.is_null(), u.email))
        .delete()
        .returning(|u| u.name)
        .into()
}

pub fn case_when_boolean() -> Query<(), (Nullable<Text>,)> {
    users
        .select(|u| when(swap!(u.email.is_null(), u.email), "none").otherwise(u.email))
        .order_by(|u| u.name)
        .into()
}

pub fn filter_boolean() -> Query<(), (Int8,)> {
    users
        .select(|u| count_star().filter(swap!(u.email.is_not_null(), u.email)))
        .into()
}

pub fn comparison_types() -> Query<(), (Text,)> {
    users
        .filter(|u| u.id.ne(swap!(u.group_id, u.name)))
        .select(|u| u.name)
        .order_by(|u| u.name)
        .into()
}

pub fn operator_operands() -> Query<(), (Nullable<Bool>,)> {
    type_samples
        .select(|s| swap!(s.v_jsonb, s.v_int4).contains(s.v_jsonb))
        .into()
}

pub fn function_arguments() -> Query<(), (Text,)> {
    users
        .select(|u| lower(swap!(u.name, u.id)))
        .order_by(|u| u.name)
        .into()
}

pub fn cast_validity() -> Query<(), (Text,)> {
    users
        .select(|u| swap!(u.id.cast::<Text>(), u.id.cast::<Bool>()))
        .order_by_output(|(id,)| id)
        .into()
}

pub fn parameter_mapping() -> Query<(Int8,), (Text,)> {
    query(|(id,)| {
        users
            .filter(|u| swap!(u.id.eq(id), u.name.eq(id)))
            .select(|u| u.name)
    })
}

pub fn insert_value_type() -> Query<(), (Text,)> {
    products
        .insert(|p| p.name("p").price_cents(swap!(10, "ten")).quantity(1))
        .returning(|p| p.name)
        .into()
}

pub fn insert_not_null() -> Query<(), (Text,)> {
    users
        .insert(|u| swap!(u.name("x"), u.email("x")))
        .returning(|u| u.name)
        .into()
}

pub fn insert_arity() -> Query<(), (Text,)> {
    users
        .insert(|u| swap!(u.name("x").email(null()), u.name("x", null())))
        .returning(|u| u.name)
        .into()
}

pub fn update_assignment_type() -> Query<(), (Text, Int4)> {
    products
        .update(|p| p.quantity.set(swap!(p.quantity + 1, p.name)))
        .returning(|p| (p.name, p.quantity))
        .into()
}

pub fn nullability_propagation() -> Query<(), (Text,)> {
    users
        .insert_from(from(users), |row, u| {
            row.name(swap!(u.name.concat("!"), u.email.concat("!")))
        })
        .returning(|u| u.name)
        .into()
}

pub fn result_row_type() -> Query<(), (Int8, Text)> {
    users
        .select(|u| swap!((u.id, u.name), (u.name, u.id)))
        .order_by(|u| u.id)
        .into()
}

pub fn returning_type() -> Query<(), (Bool, Text)> {
    users
        .insert(|u| u.name("x"))
        .returning(|u| swap!((u.id.is_not_null(), u.name), (u.id, u.name)))
        .into()
}

pub fn cte_column_type() -> Query<(), (Text,)> {
    let named = cte(users.select(|u| (u.id, u.name)));
    from(named)
        .filter(|(id, _)| swap!(id.gt(0), id.gt("zero")))
        .select(|(_, name)| name)
        .order_by(|(_, name)| name)
        .into()
}

pub fn union_column_types() -> Query<(), (Int8, Text)> {
    users
        .select(|u| (u.id, u.name))
        .union(groups.select(|g| swap!((g.id, g.name), (g.name, g.id))))
        .order_by(|(_, name)| name)
        .into()
}

pub fn union_arity() -> Query<(), (Int8, Text)> {
    users
        .select(|u| (u.id, u.name))
        .union(groups.select(|g| swap!((g.id, g.name), (g.id, g.name, g.parent_id))))
        .order_by(|(_, name)| name)
        .into()
}

pub fn scalar_subquery_shape() -> Query<(), (Text, Nullable<Text>)> {
    users
        .select(|u| {
            let group = groups
                .filter(|g| g.id.eq(u.group_id))
                .select(|g| swap!((g.name,), (g.name, g.id)));
            (u.name, group)
        })
        .order_by(|u| u.name)
        .into()
}

pub fn in_subquery_type() -> Query<(), (Text,)> {
    users
        .filter(|u| u.group_id.is_in(groups.select(|g| swap!(g.id, g.name))))
        .select(|u| u.name)
        .order_by(|u| u.name)
        .into()
}
