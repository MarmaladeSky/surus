// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn lseg() -> Query {
    t.select(t.v_lseg.length()).compile().into()
}

pub fn path() -> Query {
    t.select(t.v_path.length()).compile().into()
}
