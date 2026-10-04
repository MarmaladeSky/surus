pub mod literal;
pub mod rnd;

use bytes::BytesMut;
use postgres::error::SqlState;
use postgres::types::{Format, IsNull, ToSql, Type, to_sql_checked};
use postgres::{Client, NoTls};
use std::error::Error;
use std::sync::Once;
use std::time::Instant;

#[derive(Debug)]
pub struct TextParam(pub String);

impl ToSql for TextParam {
    fn to_sql(&self, _: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        out.extend_from_slice(self.0.as_bytes());
        Ok(IsNull::No)
    }

    fn accepts(_: &Type) -> bool {
        true
    }

    fn encode_format(&self, _: &Type) -> Format {
        Format::Text
    }

    to_sql_checked!();
}

const SCHEMA: [&str; 11] = [
    include_str!("../schema/groups.sql"),
    include_str!("../schema/users.sql"),
    include_str!("../schema/logins_days.sql"),
    include_str!("../schema/products.sql"),
    include_str!("../schema/custom_types.sql"),
    include_str!("../schema/type_samples.sql"),
    include_str!("../schema/tickets.sql"),
    include_str!("../schema/user_extensions.sql"),
    include_str!("../schema/views.sql"),
    include_str!("../schema/identifiers.sql"),
    include_str!("../schema/reservations.sql"),
];

static FIXTURES: Once = Once::new();

pub type Param = Box<dyn ToSql + Sync>;
pub type Params = Vec<Param>;

pub struct Query {
    pub sql: String,
    pub params: Params,
}

impl Query {
    pub fn plain(sql: &str) -> Query {
        Query::with_params(sql, Vec::new())
    }

    pub fn with_params(sql: &str, params: Params) -> Query {
        Query {
            sql: sql.to_string(),
            params,
        }
    }
}

pub struct Case {
    client: Client,
}

impl Case {
    pub fn new() -> Case {
        let url = std::env::var("RSLICK_TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());
        let mut client = Client::connect(&url, NoTls).expect("test database must be running");
        FIXTURES.call_once(|| {
            for sql in SCHEMA {
                client.batch_execute(sql).expect("fixture sql must succeed");
            }
        });
        client
            .batch_execute("BEGIN")
            .expect("transaction must start");
        Case { client }
    }

    pub fn exec(&mut self, sql: &str) {
        self.client
            .batch_execute(sql)
            .expect("setup sql must succeed");
    }

    pub fn exec_committed(&mut self, sql: &str) {
        self.exec("COMMIT");
        self.exec(sql);
        self.exec("BEGIN");
    }

    pub fn try_fetch(&mut self, query: &Query) -> Result<Vec<String>, postgres::Error> {
        self.try_fetch_as("row(q.*)::text", query)
    }

    pub fn try_fetch_as(
        &mut self,
        projection: &str,
        query: &Query,
    ) -> Result<Vec<String>, postgres::Error> {
        let wrapped = wrap(&query.sql, projection);
        let params: Vec<&(dyn ToSql + Sync)> = query.params.iter().map(|p| p.as_ref()).collect();
        self.exec("SAVEPOINT q");
        let rows = self
            .client
            .query(&wrapped, &params)
            .map(|rows| rows.iter().map(|row| row.get(0)).collect());
        self.exec("ROLLBACK TO SAVEPOINT q");
        rows
    }

    pub fn fetch(&mut self, query: &Query) -> Vec<String> {
        self.try_fetch(query)
            .unwrap_or_else(|e| panic!("query failed: {e:?}\n{}", query.sql))
    }

    pub fn assert_same(&mut self, plain_sql: &str, dsl: impl FnOnce() -> Query) {
        let dsl_query = compile(dsl);
        let expected = self.fetch(&Query::plain(plain_sql));
        require_rows(&expected, plain_sql);
        let actual = self.fetch(&dsl_query);
        assert_eq!(actual, expected, "dsl sql: {}", dsl_query.sql);
    }

    pub fn assert_same_named(&mut self, plain_sql: &str, dsl: impl FnOnce() -> Query) {
        let dsl_query = compile(dsl);
        let projection = "row_to_json(q)::text";
        let expected = self
            .try_fetch_as(projection, &Query::plain(plain_sql))
            .unwrap();
        require_rows(&expected, plain_sql);
        let actual = self
            .try_fetch_as(projection, &dsl_query)
            .unwrap_or_else(|e| panic!("query failed: {e:?}\n{}", dsl_query.sql));
        assert_eq!(actual, expected, "dsl sql: {}", dsl_query.sql);
    }

    pub fn outcome(&mut self, query: &Query) -> Result<Vec<String>, SqlState> {
        self.try_fetch(query)
            .map_err(|e| e.code().cloned().expect("database error must carry a code"))
    }

    pub fn assert_same_outcome(&mut self, plain_sql: &str, dsl: impl FnOnce() -> Query) {
        let dsl_query = compile(dsl);
        let expected = self.outcome(&Query::plain(plain_sql));
        if let Ok(rows) = &expected {
            require_rows(rows, plain_sql);
        }
        let actual = self.outcome(&dsl_query);
        assert_eq!(actual, expected, "dsl sql: {}", dsl_query.sql);
    }

    pub fn assert_param_types(&mut self, expected: &[Type], dsl: impl FnOnce() -> Query) {
        let dsl_query = compile(dsl);
        let statement = self
            .client
            .prepare(&dsl_query.sql)
            .unwrap_or_else(|e| panic!("prepare failed: {e:?}\n{}", dsl_query.sql));
        assert_eq!(statement.params(), expected, "dsl sql: {}", dsl_query.sql);
        assert_eq!(
            dsl_query.params.len(),
            expected.len(),
            "bound parameter count"
        );
    }

    pub fn assert_same_error(&mut self, plain_sql: &str, dsl: impl FnOnce() -> Query) {
        let dsl_query = compile(dsl);
        let expected = self
            .try_fetch(&Query::plain(plain_sql))
            .expect_err("plain sql must fail");
        let actual = match self.try_fetch(&dsl_query) {
            Err(e) => e,
            Ok(rows) => panic!("dsl sql must fail: {}\nrows: {rows:?}", dsl_query.sql),
        };
        assert_eq!(actual.code(), expected.code(), "dsl sql: {}", dsl_query.sql);
    }
}

fn wrap(sql: &str, projection: &str) -> String {
    let sql = sql.trim();
    if sql.len() > 4 && sql[..4].eq_ignore_ascii_case("with") {
        let (ctes, main) = sql.split_at(main_statement_start(sql));
        format!("{ctes}, q AS ({main}) SELECT {projection} FROM q")
    } else {
        format!("WITH q AS ({sql}) SELECT {projection} FROM q")
    }
}

#[derive(PartialEq)]
enum CteScan {
    Name,
    AfterName,
    AfterAs,
    AfterBody,
    SearchOrCycle,
}

fn main_statement_start(sql: &str) -> usize {
    const MAIN: [&str; 7] = [
        "select", "insert", "update", "delete", "merge", "values", "table",
    ];
    let bytes = sql.as_bytes();
    let mut depth = 0;
    let mut state = CteScan::Name;
    let mut i = 4;
    while i < bytes.len() {
        match bytes[i] {
            quote @ (b'\'' | b'"') => {
                i += 1;
                while i < bytes.len() && bytes[i] != quote {
                    i += 1;
                }
            }
            b'(' => {
                if depth == 0 && matches!(state, CteScan::AfterBody | CteScan::SearchOrCycle) {
                    return i;
                }
                depth += 1;
            }
            b')' => {
                depth -= 1;
                if depth == 0 && state == CteScan::AfterAs {
                    state = CteScan::AfterBody;
                }
            }
            b',' if depth == 0 => state = CteScan::Name,
            c if depth == 0 && (c.is_ascii_alphabetic() || c == b'_') => {
                let start = i;
                while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
                let word = sql[start..i].to_ascii_lowercase();
                match state {
                    CteScan::Name if word != "recursive" => state = CteScan::AfterName,
                    CteScan::AfterName if word == "as" => state = CteScan::AfterAs,
                    CteScan::AfterBody if word == "search" || word == "cycle" => {
                        state = CteScan::SearchOrCycle
                    }
                    CteScan::AfterBody => return start,
                    CteScan::SearchOrCycle if MAIN.contains(&word.as_str()) => return start,
                    _ => {}
                }
                continue;
            }
            _ => {}
        }
        i += 1;
    }
    panic!("no main statement after WITH clause: {sql}");
}

fn require_rows(rows: &[String], plain_sql: &str) {
    assert!(
        !rows.is_empty(),
        "plain query returns no rows, so the test cannot fail on a placeholder:\n{plain_sql}"
    );
}

fn compile(dsl: impl FnOnce() -> Query) -> Query {
    let started = Instant::now();
    let query = dsl();
    let test = std::thread::current().name().unwrap_or("?").to_string();
    println!("[{test}] dsl compiled in {:?}", started.elapsed());
    query
}

impl Drop for Case {
    fn drop(&mut self) {
        let _ = self.client.batch_execute("ROLLBACK");
    }
}
