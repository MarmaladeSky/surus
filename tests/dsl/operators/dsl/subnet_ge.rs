use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn inet_inet() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}
