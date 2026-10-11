use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn float8() -> Query<(), (t::Nullable<t::Float8>,)> {
    placeholder()
}
