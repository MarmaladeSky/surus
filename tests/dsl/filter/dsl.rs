use crate::schema::users;
use surus::prelude::*;
use surus::types::{Int8, Text};
use surus::{Query, query};

pub fn by_name() -> Query<(Text,), (Int8, Text)> {
    query(|(name,)| users.filter(|u| u.name.eq(name)).select(|u| (u.id, u.name)))
}
