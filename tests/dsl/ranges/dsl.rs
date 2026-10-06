use crate::support::Query;
use schema::*;
use surus::*;

pub fn constructors_and_accessors() -> Query {
    let r = type_samples.v_int4range;
    type_samples
        .select((
            int4range(1, 10).bounds("[]"),
            int4range(lower(r), upper(r)).bounds("(]"),
            lower(r),
            upper(r),
            isempty(r),
            lower_inc(r),
            upper_inc(r),
            lower(type_samples.v_tsrange),
        ))
        .order_by(r)
        .compile()
        .into()
}

pub fn containment_and_overlap() -> Query {
    let r = type_samples.v_int4range;
    type_samples
        .select((
            r.contains(lower(r)),
            r.contained_by(int4range(-100000, 100000)),
            r.overlaps(int4range(lower(r), lower(r) + 1)),
            r.adjacent(int4range(upper(r), upper(r) + 5)),
            r * int4range(lower(r) + 1, null()),
            r + int4range(upper(r), upper(r) + 5),
        ))
        .order_by(r)
        .compile()
        .into()
}

pub fn multirange_functions() -> Query {
    let m = type_samples.v_int4multirange;
    let ranges = unnest(m);
    let (range,) = ranges.columns();
    type_samples
        .cross_join(ranges)
        .select((
            lower(m),
            upper(m),
            m.contains(lower(m)),
            m - int4multirange(int4range(lower(m), lower(m) + 1)),
            range,
        ))
        .order_by((m, range))
        .compile()
        .into()
}

pub fn range_aggregates() -> Query {
    let t = type_samples;
    t.select((
        range_agg(t.v_int4range),
        range_intersect_agg(t.v_int4range),
        range_agg(t.v_int4multirange),
    ))
    .compile()
    .into()
}
