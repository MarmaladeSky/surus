use crate::support::{Param, Query, TextParam};
use schema::*;
use surus::*;

/// Hands compiled statements to the test driver; shared by all `dsl` modules.
impl<R> From<Compiled<R>> for Query {
    fn from(compiled: Compiled<R>) -> Query {
        let params = compiled
            .params
            .into_iter()
            .map(|param| -> Param {
                match param.value {
                    Value::Text(text) => Box::new(Some(TextParam(text))),
                    Value::Null | Value::Placeholder => Box::new(None::<TextParam>),
                }
            })
            .collect();
        Query::with_params(&compiled.sql, params)
    }
}

pub fn single_row() -> Query {
    users
        .select((users.id, users.name, users.email, users.group_id))
        .compile()
        .into()
}

pub fn distinct_group_ids() -> Query {
    users
        .select(users.group_id)
        .distinct()
        .order_by(users.group_id)
        .compile()
        .into()
}

pub fn distinct_on_first_user_per_group() -> Query {
    users
        .select((users.group_id, users.name))
        .distinct_on(users.group_id)
        .order_by((users.group_id, users.name))
        .compile()
        .into()
}

pub fn limit_offset(limit: i64, offset: i64) -> Query {
    users
        .select(users.name)
        .order_by(users.name)
        .limit(limit)
        .offset(offset)
        .compile()
        .into()
}

pub fn fetch_first_rows_only(count: i64, offset: i64) -> Query {
    users
        .select(users.name)
        .order_by(users.name)
        .offset(offset)
        .fetch_first(count)
        .compile()
        .into()
}

pub fn fetch_first_with_ties(count: i64) -> Query {
    users
        .select(users.group_id)
        .order_by(users.group_id)
        .fetch_first_with_ties(count)
        .compile()
        .into()
}

pub fn aliases() -> Query {
    let u = alias!(users, "u");
    let g = alias!(groups, "g");
    u.join(g, g.id.eq(u.group_id))
        .select((u.name.as_("user_name"), g.name.as_("group_name")))
        .order_by(u.name)
        .compile()
        .into()
}

pub fn for_update(batch: &str) -> Query {
    tickets
        .select(tickets.name)
        .filter(tickets.batch.eq(batch))
        .order_by(tickets.name)
        .for_update()
        .compile()
        .into()
}

pub fn for_share(batch: &str) -> Query {
    tickets
        .select(tickets.name)
        .filter(tickets.batch.eq(batch))
        .order_by(tickets.name)
        .for_share()
        .compile()
        .into()
}

pub fn for_update_skip_locked(batch: &str) -> Query {
    tickets
        .select(tickets.name)
        .filter(tickets.batch.eq(batch))
        .order_by(tickets.name)
        .for_update()
        .skip_locked()
        .compile()
        .into()
}

pub fn for_update_nowait(name: &str) -> Query {
    tickets
        .select(tickets.name)
        .filter(tickets.name.eq(name))
        .for_update()
        .nowait()
        .compile()
        .into()
}

pub fn star() -> Query {
    users.order_by(users.name).compile().into()
}

pub fn table_star() -> Query {
    let u = alias!(users, "u");
    let g = alias!(groups, "g");
    u.join(g, g.id.eq(u.group_id))
        .select((u.star(), g.name.as_("group_name")))
        .order_by(u.name)
        .compile()
        .into()
}

pub fn without_from() -> Query {
    select((
        param(1) + 1,
        param("a").concat("b"),
        param(10).gt(3),
        current_date().eq(current_date()),
    ))
    .compile()
    .into()
}

pub fn for_key_share(batch: &str) -> Query {
    tickets
        .select(tickets.name)
        .filter(tickets.batch.eq(batch))
        .order_by(tickets.name)
        .for_key_share()
        .compile()
        .into()
}

pub fn for_no_key_update_skip_locked(batch: &str) -> Query {
    tickets
        .select(tickets.name)
        .filter(tickets.batch.eq(batch))
        .order_by(tickets.name)
        .for_no_key_update()
        .skip_locked()
        .compile()
        .into()
}

pub fn for_update_of_table() -> Query {
    let u = alias!(users, "u");
    let g = alias!(groups, "g");
    u.join(g, g.id.eq(u.group_id))
        .select((u.name, g.name))
        .order_by(u.name)
        .for_update()
        .of(&u)
        .compile()
        .into()
}
