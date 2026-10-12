use crate::schema::{PositiveInt, products, type_samples};
use surus::prelude::*;
use surus::types::{Nullable, Regtype};
use surus::{Query, pg_typeof, select, value};

pub fn cast_to_domain() -> Query<(), (PositiveInt, PositiveInt, Regtype)> {
    products
        .select(|p| {
            (
                (p.quantity + 1).cast::<PositiveInt>(),
                (p.price_cents + 1).cast::<PositiveInt>(),
                pg_typeof((p.quantity + 1).cast::<PositiveInt>()),
            )
        })
        .into()
}

pub fn check_violation_on_cast() -> Query<(), (PositiveInt,)> {
    select(value(0).cast::<PositiveInt>()).into()
}

pub fn check_violation_on_insert(value: i32) -> Query<(), (Nullable<PositiveInt>,)> {
    type_samples
        .insert(|s| s.v_positive_int(value))
        .returning(|s| s.v_positive_int)
        .into()
}
