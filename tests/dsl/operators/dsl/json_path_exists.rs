use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn jsonb_jsonpath() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}
