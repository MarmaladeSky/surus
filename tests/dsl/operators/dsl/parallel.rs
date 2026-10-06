// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn line_line() -> Query {
    t.select(t.v_line.parallel(t.v_line)).compile().into()
}

pub fn lseg_lseg() -> Query {
    t.select(t.v_lseg.parallel(t.v_lseg)).compile().into()
}
