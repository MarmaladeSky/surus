use crate::support::Query;
use schema::*;
use surus::*;

pub fn declaration_order() -> Query {
    type_samples
        .select(type_samples.v_mood)
        .order_by(type_samples.v_mood)
        .compile()
        .into()
}

pub fn comparison_with_parameter(mood: &str) -> Query {
    type_samples
        .select(type_samples.v_mood)
        .filter(type_samples.v_mood.ge(param(mood).cast::<Mood>()))
        .order_by(type_samples.v_mood)
        .compile()
        .into()
}

pub fn enum_functions() -> Query {
    let mood = type_samples.v_mood;
    type_samples
        .select((
            mood,
            enum_range_between(mood, null()),
            enum_first(mood),
            enum_last(mood),
            enum_range::<Mood>(),
        ))
        .order_by(mood)
        .compile()
        .into()
}
