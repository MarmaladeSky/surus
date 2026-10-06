use crate::support::Query;
use schema::*;
use surus::*;

pub fn by_name(name: &str) -> Query {
    users
        .delete()
        .filter(users.name.eq(name))
        .returning(users.name)
        .compile()
        .into()
}

pub fn delete_using(group: &str) -> Query {
    users
        .delete()
        .using(groups)
        .filter(groups.id.eq(users.group_id).and(groups.name.eq(group)))
        .returning(users.name)
        .compile()
        .into()
}

pub fn returning_table_star(group: &str) -> Query {
    users
        .delete()
        .using(groups)
        .filter(groups.id.eq(users.group_id).and(groups.name.eq(group)))
        .returning((users.star(), groups.name.as_("group_name")))
        .compile()
        .into()
}

pub fn returning_old(name: &str) -> Query {
    let (old, new) = (users.old(), users.new());
    users
        .delete()
        .filter(users.name.eq(name))
        .returning((old.name, old.email, new.name.is_null()))
        .compile()
        .into()
}

pub fn delete_using_subquery() -> Query {
    let roots = groups.select(groups.id).filter(groups.parent_id.is_null());
    users
        .delete()
        .filter(users.group_id.in_(roots))
        .returning(users.name)
        .compile()
        .into()
}
