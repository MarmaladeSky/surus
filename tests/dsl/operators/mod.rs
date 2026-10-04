mod catalog;
mod dsl;

use crate::support::{Case, Query, literal};
use postgres::error::SqlState;
use std::panic::{AssertUnwindSafe, catch_unwind};

pub struct Pair {
    pub left: Option<&'static str>,
    pub right: &'static str,
    pub dsl: fn() -> Query,
}

pub fn run(symbol: &str, pairs: &[Pair]) {
    let mut case = Case::new();
    let columns: Vec<String> = literal::ALL.iter().map(|(t, _)| format!("v_{t}")).collect();
    let values: Vec<String> = literal::ALL
        .iter()
        .map(|(_, f)| format!("'{}'", f()))
        .collect();
    case.exec(&format!(
        "INSERT INTO type_samples ({}) VALUES ({})",
        columns.join(", "),
        values.join(", ")
    ));

    let failed: Vec<String> = pairs
        .iter()
        .filter(|p| {
            let plain = match p.left {
                Some(left) => format!("SELECT {left} {symbol} {} FROM type_samples", p.right),
                None => format!("SELECT {symbol} {} FROM type_samples", p.right),
            };
            catch_unwind(AssertUnwindSafe(|| {
                let expected = case.outcome(&Query::plain(&plain));
                assert_ne!(
                    expected,
                    Err(SqlState::UNDEFINED_FUNCTION),
                    "catalog signature is not accepted by the server: {plain}"
                );
                case.assert_same_outcome(&plain, p.dsl)
            }))
            .is_err()
        })
        .map(|p| format!("{} {symbol} {}", p.left.unwrap_or(""), p.right))
        .collect();
    assert!(failed.is_empty(), "failed signatures: {failed:#?}");
}
