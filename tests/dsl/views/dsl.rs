use crate::support::Query;
use schema::*;
use surus::*;

pub fn select_from_view() -> Query {
    users_with_groups
        .select((users_with_groups.name, users_with_groups.group_name))
        .order_by(users_with_groups.name)
        .compile()
        .into()
}

pub fn insert_through_updatable_view(name: &str, group: &str) -> Query {
    let group_id = groups.select(groups.id).filter(groups.name.eq(group));
    grouped_users
        .insert((grouped_users.name, grouped_users.group_id))
        .values((name, group_id))
        .returning((
            grouped_users.name,
            grouped_users.email,
            grouped_users.group_id.is_not_null(),
        ))
        .compile()
        .into()
}

pub fn check_option_violation(name: &str) -> Query {
    grouped_users
        .insert((grouped_users.name,))
        .values((name,))
        .returning(grouped_users.name)
        .compile()
        .into()
}
