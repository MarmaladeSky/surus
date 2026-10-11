use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn jsonb_text() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}
