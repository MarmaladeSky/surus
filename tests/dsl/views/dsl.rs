use crate::schema::{grouped_users, groups, users_with_groups};
use surus::prelude::*;
use surus::types::{Bool, Nullable, Text};
use surus::{Query, query};

pub fn select_from_view() -> Query<(), (Text, Text)> {
    users_with_groups
        .select(|u| (u.name, u.group_name))
        .order_by(|u| u.name)
        .into()
}

pub fn insert_through_updatable_view() -> Query<(Text, Text), (Text, Nullable<Text>, Bool)> {
    query(|(name, group)| {
        let group_id = groups.filter(|g| g.name.eq(group)).select(|g| g.id);
        grouped_users
            .insert(|u| u.name(name).group_id(group_id))
            .returning(|u| (u.name, u.email, u.group_id.is_not_null()))
    })
}

pub fn check_option_violation() -> Query<(Text,), (Text,)> {
    query(|(name,)| grouped_users.insert(|u| u.name(name)).returning(|u| u.name))
}
