use crate::support::Query;
use schema::*;
use surus::*;

pub fn scalar_group_name() -> Query {
    let group_name = groups
        .select(groups.name)
        .filter(groups.id.eq(users.group_id));
    users
        .select((users.name, group_name))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn scalar_multiple_rows_fails() -> Query {
    users
        .select((users.name, groups.select(groups.name)))
        .compile()
        .into()
}

pub fn exists_groups_with_users() -> Query {
    let members = users.select(1).filter(users.group_id.eq(groups.id));
    groups
        .select(groups.name)
        .filter(exists(members))
        .order_by(groups.name)
        .compile()
        .into()
}

pub fn not_exists_empty_groups() -> Query {
    let members = users.select(1).filter(users.group_id.eq(groups.id));
    groups
        .select(groups.name)
        .filter(!exists(members))
        .order_by(groups.name)
        .compile()
        .into()
}

pub fn in_subquery(group: &str) -> Query {
    let ids = groups.select(groups.id).filter(groups.name.eq(group));
    users
        .select(users.name)
        .filter(users.group_id.in_(ids))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn any_not_the_cheapest() -> Query {
    let other = alias!(products, "other");
    let prices = other.select(other.price_cents);
    products
        .select(products.name)
        .filter(products.price_cents.gt(any(prices)))
        .order_by(products.name)
        .compile()
        .into()
}

pub fn all_the_most_expensive() -> Query {
    let other = alias!(products, "other");
    let prices = other.select(other.price_cents);
    products
        .select(products.name)
        .filter(products.price_cents.ge(all(prices)))
        .order_by(products.name)
        .compile()
        .into()
}

pub fn not_in_subquery(group: &str) -> Query {
    let ids = groups.select(groups.id).filter(groups.name.eq(group));
    users
        .select(users.name)
        .filter(users.group_id.not_in(ids))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn not_in_with_null_returns_nothing() -> Query {
    let other = alias!(users, "other");
    let with_null = other.select(other.group_id);
    let without_null = other
        .select(other.group_id)
        .filter(other.group_id.is_not_null());
    users
        .select(("with null", count_star()))
        .filter(users.group_id.not_in(with_null))
        .union_all(
            users
                .select(("without null", count_star()))
                .filter(users.group_id.not_in(without_null)),
        )
        .compile()
        .into()
}

pub fn row_valued_in(group: &str) -> Query {
    let pairs = groups.select((groups.id, groups.name.eq(group)));
    users
        .select(users.name)
        .filter(row((users.group_id, true)).in_(pairs))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn correlated_scalar_in_arithmetic() -> Query {
    let g = alias!(groups, "g");
    let u = alias!(users, "u");
    let members = u.select(count_star()).filter(u.group_id.eq(g.id));
    g.select((g.name, members + 1))
        .order_by(g.name)
        .compile()
        .into()
}

pub fn exists_and_predicate() -> Query {
    let g = alias!(groups, "g");
    let u = alias!(users, "u");
    let members = u.select(1).filter(u.group_id.eq(g.id));
    g.select(g.name)
        .filter(exists(members).and(g.parent_id.is_null()))
        .order_by(g.name)
        .compile()
        .into()
}

pub fn scalar_subquery_compared() -> Query {
    let other = alias!(products, "other");
    let average = other.select(avg(other.price_cents));
    products
        .select(products.name)
        .filter(products.price_cents.gt(average))
        .order_by(products.name)
        .compile()
        .into()
}
