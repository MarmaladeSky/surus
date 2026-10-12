use crate::schema::{groups, users};
use surus::prelude::*;
use surus::types::{Bool, Int8, Nullable, Text};
use surus::{Query, query};

pub fn by_name() -> Query<(Text,), (Text,)> {
    query(|(name,)| {
        users
            .filter(|u| u.name.eq(name))
            .delete()
            .returning(|u| u.name)
    })
}

pub fn delete_using() -> Query<(Text,), (Text,)> {
    query(|(group,)| {
        users
            .join(groups, |(u, g)| g.id.eq(u.group_id))
            .filter(|(_, g)| g.name.eq(group))
            .delete()
            .returning(|(u, _)| u.name)
    })
}

pub fn returning_table_star() -> Query<(Text,), (Int8, Text, Nullable<Text>, Nullable<Int8>, Text)>
{
    query(|(group,)| {
        users
            .join(groups, |(u, g)| g.id.eq(u.group_id))
            .filter(|(_, g)| g.name.eq(group))
            .delete()
            .returning(|(u, g)| (u.star(), g.name.alias("group_name")))
    })
}

pub fn returning_old() -> Query<(Text,), (Text, Nullable<Text>, Bool)> {
    query(|(name,)| {
        users
            .filter(|u| u.name.eq(name))
            .delete()
            .returning_old_new(|(old, new)| (old.name, old.email, new.name.is_null()))
    })
}

pub fn delete_using_subquery() -> Query<(), (Text,)> {
    let roots = groups.filter(|g| g.parent_id.is_null()).select(|g| g.id);
    users
        .filter(|u| u.group_id.is_in(roots))
        .delete()
        .returning(|u| u.name)
        .into()
}
