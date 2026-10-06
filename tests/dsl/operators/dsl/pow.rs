// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn float8_float8() -> Query {
    t.select(t.v_float8.pow(t.v_float8)).compile().into()
}

pub fn numeric_numeric() -> Query {
    t.select(t.v_numeric.pow(t.v_numeric)).compile().into()
}
