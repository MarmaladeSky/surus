mod catalog;
mod dsl;

use crate::support::{Case, Compiled, literal, sql};
use postgres::error::SqlState;
use std::panic::{AssertUnwindSafe, catch_unwind};
use surus::{Query, Row};

pub struct Run {
    case: Case,
    symbol: &'static str,
    failed: Vec<String>,
}

impl Run {
    pub fn new(symbol: &'static str) -> Run {
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
        Run {
            case,
            symbol,
            failed: Vec::new(),
        }
    }

    pub fn pair<R: Row>(&mut self, left: Option<&str>, right: &str, dsl: fn() -> Query<(), R>) {
        let symbol = self.symbol;
        let plain = match left {
            Some(left) => format!("SELECT {left} {symbol} {right} FROM type_samples"),
            None => format!("SELECT {symbol} {right} FROM type_samples"),
        };
        let case = &mut self.case;
        let passed = catch_unwind(AssertUnwindSafe(|| {
            let expected = case.outcome(&Compiled::plain(&plain));
            assert_ne!(
                expected,
                Err(SqlState::UNDEFINED_FUNCTION),
                "catalog signature is not accepted by the server: {plain}"
            );
            case.assert_same_outcome(sql(&plain), dsl(), ())
        }))
        .is_ok();
        if !passed {
            self.failed
                .push(format!("{} {symbol} {right}", left.unwrap_or("")));
        }
    }

    pub fn finish(self) {
        assert!(
            self.failed.is_empty(),
            "failed signatures: {:#?}",
            self.failed
        );
    }
}
