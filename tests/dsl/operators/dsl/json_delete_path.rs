use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn jsonb_text_array() -> Query<(), (t::Nullable<t::Jsonb>,)> {
    placeholder()
}
