use crate::schema::type_samples;
use surus::prelude::*;
use surus::types::{Bool, Int4, Multirange, Nullable, Range, Timestamp};
use surus::{
    Query, isempty, lower, lower_inc, multirange, null, range, range_agg, range_bounds,
    range_intersect_agg, unnest, upper, upper_inc,
};

pub fn constructors_and_accessors() -> Query<
    (),
    (
        Range<Int4>,
        Nullable<Range<Int4>>,
        Nullable<Int4>,
        Nullable<Int4>,
        Nullable<Bool>,
        Nullable<Bool>,
        Nullable<Bool>,
        Nullable<Timestamp>,
    ),
> {
    type_samples
        .select(|s| {
            let r = s.v_int4range;
            (
                range_bounds(1, 10, "[]"),
                range_bounds(lower(r), upper(r), "(]"),
                lower(r),
                upper(r),
                isempty(r),
                lower_inc(r),
                upper_inc(r),
                lower(s.v_tsrange),
            )
        })
        .order_by(|s| s.v_int4range)
        .into()
}

pub fn containment_and_overlap() -> Query<
    (),
    (
        Nullable<Bool>,
        Nullable<Bool>,
        Nullable<Bool>,
        Nullable<Bool>,
        Nullable<Range<Int4>>,
        Nullable<Range<Int4>>,
    ),
> {
    type_samples
        .select(|s| {
            let r = s.v_int4range;
            (
                r.contains(lower(r)),
                r.contained_by(range(-100000, 100000)),
                r.overlaps(range(lower(r), lower(r) + 1)),
                r.adjacent(range(upper(r), upper(r) + 5)),
                r.intersection(range(lower(r) + 1, null())),
                r.union(range(upper(r), upper(r) + 5)),
            )
        })
        .order_by(|s| s.v_int4range)
        .into()
}

pub fn multirange_functions() -> Query<
    (),
    (
        Nullable<Int4>,
        Nullable<Int4>,
        Nullable<Bool>,
        Nullable<Multirange<Int4>>,
        Nullable<Range<Int4>>,
    ),
> {
    type_samples
        .select(|s| {
            let m = s.v_int4multirange;
            (
                lower(m),
                upper(m),
                m.contains(lower(m)),
                m.difference(multirange(range(lower(m), lower(m) + 1))),
                unnest(m),
            )
        })
        .order_by(|s| s.v_int4multirange)
        .order_by_output(|(.., ranges)| ranges)
        .into()
}

pub fn range_aggregates() -> Query<
    (),
    (
        Nullable<Multirange<Int4>>,
        Nullable<Range<Int4>>,
        Nullable<Multirange<Int4>>,
    ),
> {
    type_samples
        .select(|s| {
            (
                range_agg(s.v_int4range),
                range_intersect_agg(s.v_int4range),
                range_agg(s.v_int4multirange),
            )
        })
        .into()
}
