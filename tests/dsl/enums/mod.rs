mod dsl;

use crate::schema::Mood;
use crate::support::{Case, compile, sql};
use surus::Query;
use surus::types::{Array, Nullable};

fn seed(case: &mut Case) {
    case.exec("INSERT INTO type_samples (v_mood) VALUES ('happy'), ('sad'), ('ok'), ('happy')");
}

#[test]
fn declaration_order() {
    let mut case = Case::new();
    seed(&mut case);

    let expected: Query<(), (Nullable<Mood>,)> =
        sql("SELECT v_mood FROM type_samples ORDER BY v_mood");
    case.assert_same(expected, compile(dsl::declaration_order), ());
}

#[test]
fn comparison_with_parameter() {
    let mut case = Case::new();
    seed(&mut case);

    let expected: Query<(), (Nullable<Mood>,)> =
        sql("SELECT v_mood FROM type_samples WHERE v_mood >= 'ok' ORDER BY v_mood");
    case.assert_same(
        expected,
        compile(|| dsl::comparison_with_parameter("ok")),
        (),
    );
}

#[test]
fn enum_functions() {
    let mut case = Case::new();
    seed(&mut case);

    let expected: Query<
        (),
        (
            Nullable<Mood>,
            Nullable<Array<Mood>>,
            Nullable<Mood>,
            Nullable<Mood>,
            Nullable<Array<Mood>>,
        ),
    > = sql(
        "SELECT v_mood, enum_range(v_mood, NULL), enum_first(v_mood), enum_last(v_mood), enum_range(NULL::mood)
         FROM type_samples
         ORDER BY v_mood",
    );
    case.assert_same(expected, compile(dsl::enum_functions), ());
}
