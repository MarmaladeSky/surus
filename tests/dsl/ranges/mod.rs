mod dsl;

use crate::support::{Case, compile, literal, sql};
use surus::Query;
use surus::types::{Bool, Int4, Multirange, Nullable, Range, Timestamp};

fn seed(case: &mut Case) {
    case.exec(&format!(
        "INSERT INTO type_samples (v_int4range, v_tsrange, v_int4multirange)
         VALUES ('{}', '{}', '{}'), ('{}', '{}', '{}')",
        literal::int4range(),
        literal::tsrange(),
        literal::int4multirange(),
        literal::int4range(),
        literal::tsrange(),
        literal::int4multirange(),
    ));
}

#[test]
fn constructors_and_accessors() {
    let mut case = Case::new();
    seed(&mut case);

    let expected: Query<
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
    > = sql(
        "SELECT int4range(1, 10, '[]'), int4range(lower(v_int4range), upper(v_int4range), '(]'),
                lower(v_int4range), upper(v_int4range), isempty(v_int4range),
                lower_inc(v_int4range), upper_inc(v_int4range), lower(v_tsrange)
         FROM type_samples
         ORDER BY v_int4range",
    );
    case.assert_same(expected, compile(dsl::constructors_and_accessors), ());
}

#[test]
fn containment_and_overlap() {
    let mut case = Case::new();
    seed(&mut case);

    let expected: Query<
        (),
        (
            Nullable<Bool>,
            Nullable<Bool>,
            Nullable<Bool>,
            Nullable<Bool>,
            Nullable<Range<Int4>>,
            Nullable<Range<Int4>>,
        ),
    > = sql("SELECT v_int4range @> lower(v_int4range),
                v_int4range <@ int4range(-100000, 100000),
                v_int4range && int4range(lower(v_int4range), lower(v_int4range) + 1),
                v_int4range -|- int4range(upper(v_int4range), upper(v_int4range) + 5),
                v_int4range * int4range(lower(v_int4range) + 1, NULL),
                v_int4range + int4range(upper(v_int4range), upper(v_int4range) + 5)
         FROM type_samples
         ORDER BY v_int4range");
    case.assert_same(expected, compile(dsl::containment_and_overlap), ());
}

#[test]
fn multirange_functions() {
    let mut case = Case::new();
    seed(&mut case);

    let expected: Query<
        (),
        (
            Nullable<Int4>,
            Nullable<Int4>,
            Nullable<Bool>,
            Nullable<Multirange<Int4>>,
            Nullable<Range<Int4>>,
        ),
    > = sql(
        "SELECT lower(v_int4multirange), upper(v_int4multirange),
                v_int4multirange @> lower(v_int4multirange),
                v_int4multirange - int4multirange(int4range(lower(v_int4multirange), lower(v_int4multirange) + 1)),
                unnest(v_int4multirange)
         FROM type_samples
         ORDER BY v_int4multirange, 5",
    );
    case.assert_same(expected, compile(dsl::multirange_functions), ());
}

#[test]
fn range_aggregates() {
    let mut case = Case::new();
    seed(&mut case);

    let expected: Query<
        (),
        (
            Nullable<Multirange<Int4>>,
            Nullable<Range<Int4>>,
            Nullable<Multirange<Int4>>,
        ),
    > = sql(
        "SELECT range_agg(v_int4range), range_intersect_agg(v_int4range), range_agg(v_int4multirange)
         FROM type_samples",
    );
    case.assert_same(expected, compile(dsl::range_aggregates), ());
}
