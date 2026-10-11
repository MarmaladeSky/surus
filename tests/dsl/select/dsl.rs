use crate::support::placeholder;
use surus::Query;
use surus::types::{Bool, Int4, Int8, Nullable, Text};

pub fn single_row() -> Query<(), (Int8, Text, Nullable<Text>, Nullable<Int8>)> {
    placeholder()
}

pub fn distinct_group_ids() -> Query<(), (Nullable<Int8>,)> {
    placeholder()
}

pub fn distinct_on_first_user_per_group() -> Query<(), (Nullable<Int8>, Text)> {
    placeholder()
}

pub fn limit_offset(_limit: i64, _offset: i64) -> Query<(), (Text,)> {
    placeholder()
}

pub fn fetch_first_rows_only(_count: i64, _offset: i64) -> Query<(), (Text,)> {
    placeholder()
}

pub fn fetch_first_with_ties(_count: i64) -> Query<(), (Nullable<Int8>,)> {
    placeholder()
}

pub fn aliases() -> Query<(), (Text, Text)> {
    placeholder()
}

pub fn for_update() -> Query<(Text,), (Text,)> {
    placeholder()
}

pub fn for_share() -> Query<(Text,), (Text,)> {
    placeholder()
}

pub fn for_update_skip_locked() -> Query<(Text,), (Text,)> {
    placeholder()
}

pub fn for_update_nowait() -> Query<(Text,), (Text,)> {
    placeholder()
}

pub fn star() -> Query<(), (Int8, Text, Nullable<Text>, Nullable<Int8>)> {
    placeholder()
}

pub fn table_star() -> Query<(), (Int8, Text, Nullable<Text>, Nullable<Int8>, Text)> {
    placeholder()
}

pub fn without_from() -> Query<(), (Int4, Text, Bool, Bool)> {
    placeholder()
}

pub fn for_key_share() -> Query<(Text,), (Text,)> {
    placeholder()
}

pub fn for_no_key_update_skip_locked() -> Query<(Text,), (Text,)> {
    placeholder()
}

pub fn for_update_of_table() -> Query<(), (Text, Text)> {
    placeholder()
}
