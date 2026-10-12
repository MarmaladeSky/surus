use crate::schema::users;
use surus::prelude::*;
use surus::types::{Int4, Text};
use surus::{Query, query, values};

pub fn standalone() -> Query<(Int4, Text), (Int4, Text)> {
    query(|(number, text)| values((number, text)).row((number + 1, text)))
}

pub fn in_from_joined_with_table() -> Query<(Text, Text), (Text, Text)> {
    query(|(first, second)| {
        let labels = values((first, "first")).row((second, "second"));
        users
            .join(labels, |(u, (name, _))| name.eq(u.name))
            .select(|(u, (_, label))| (u.name, label))
            .order_by(|(_, (_, label))| label)
    })
}
