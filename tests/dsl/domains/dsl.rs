use crate::support::Query;
use schema::*;
use surus::*;

pub fn cast_to_domain() -> Query {
    let positive = (products.quantity + 1).cast::<PositiveInt>();
    products
        .select((
            positive.clone(),
            (products.price_cents + 1).cast::<PositiveInt>(),
            pg_typeof(positive),
        ))
        .compile()
        .into()
}

pub fn check_violation_on_cast() -> Query {
    select(param(0).cast::<PositiveInt>()).compile().into()
}

pub fn check_violation_on_insert(value: i32) -> Query {
    type_samples
        .insert((type_samples.v_positive_int,))
        .values((value,))
        .returning(type_samples.v_positive_int)
        .compile()
        .into()
}
