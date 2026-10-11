pub mod literal;
pub mod rnd;

use bytes::BytesMut;
use postgres::error::SqlState;
use postgres::types::{Format, IsNull, Kind, ToSql, Type, to_sql_checked};
use postgres::{Client, NoTls, Statement};
use std::error::Error;
use std::sync::Once;
use std::time::Instant;
use surus::types::PgType;
use surus::{Bind, Row, ToPg};

#[derive(Debug)]
pub struct TextParam(pub Option<String>);

impl ToSql for TextParam {
    fn to_sql(&self, _: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        match &self.0 {
            Some(text) => {
                out.extend_from_slice(text.as_bytes());
                Ok(IsNull::No)
            }
            None => Ok(IsNull::Yes),
        }
    }

    fn accepts(_: &Type) -> bool {
        true
    }

    fn encode_format(&self, _: &Type) -> Format {
        Format::Text
    }

    to_sql_checked!();
}

#[derive(Debug, Clone)]
pub struct Input(pub String);

impl<T: PgType> ToPg<T> for Input {
    fn to_pg(&self) -> Option<String> {
        Some(self.0.clone())
    }
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

type Param = Box<dyn ToSql + Sync>;

pub struct Compiled {
    sql: String,
    params: Vec<Param>,
    types: Vec<String>,
    declared: bool,
}

impl Compiled {
    pub fn plain(sql: &str) -> Compiled {
        Compiled {
            sql: sql.to_string(),
            params: Vec::new(),
            types: Vec::new(),
            declared: false,
        }
    }

    fn typed<R>(query: &surus::Query<(), R>, declared: bool) -> Compiled {
        Compiled {
            sql: query.sql().to_string(),
            params: query
                .args()
                .iter()
                .map(|arg| Box::new(TextParam(arg.value.clone())) as Param)
                .collect(),
            types: query.args().iter().map(|arg| arg.ty.clone()).collect(),
            declared,
        }
    }
}

pub fn sql<P, R>(text: &str) -> surus::Query<P, R> {
    surus::Query::new(text.to_string(), Vec::new())
}

pub fn placeholder<P, R>() -> surus::Query<P, R> {
    sql("SELECT NULL WHERE false")
}

pub fn compile<Q>(dsl: impl FnOnce() -> Q) -> Q {
    let started = Instant::now();
    let query = dsl();
    let test = std::thread::current().name().unwrap_or("?").to_string();
    println!("[{test}] dsl compiled in {:?}", started.elapsed());
    query
}

pub struct Case {
    client: Client,
}

impl Case {
    pub fn new() -> Case {
        let url = std::env::var("SURUS_TEST_DATABASE_URL")
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

    fn oids(&mut self, names: &[String]) -> Vec<u32> {
        names
            .iter()
            .map(|name| {
                let oid: Option<u32> = self
                    .client
                    .query_one("SELECT to_regtype($1)::oid", &[name])
                    .expect("type lookup must succeed")
                    .get(0);
                oid.unwrap_or_else(|| panic!("unknown type {name}"))
            })
            .collect()
    }

    fn types(&mut self, query: &Compiled) -> Vec<Type> {
        if !query.declared {
            return Vec::new();
        }
        self.oids(&query.types)
            .into_iter()
            .zip(&query.types)
            .map(|(oid, name)| {
                Type::from_oid(oid)
                    .unwrap_or_else(|| Type::new(name.clone(), oid, Kind::Simple, String::new()))
            })
            .collect()
    }

    fn try_prepare(&mut self, sql: &str, query: &Compiled) -> Result<Statement, postgres::Error> {
        let types = self.types(query);
        self.exec("SAVEPOINT p");
        let statement = self.client.prepare_typed(sql, &types);
        self.exec("ROLLBACK TO SAVEPOINT p");
        statement
    }

    fn assert_shape(&mut self, query: &Compiled, columns: &[String], lenient: bool) {
        let statement = match self.try_prepare(&query.sql, query) {
            Ok(statement) => statement,
            Err(_) if lenient => return,
            Err(e) => panic!("prepare failed: {e:?}\n{}", query.sql),
        };
        let params: Vec<u32> = statement.params().iter().map(Type::oid).collect();
        assert_eq!(
            params,
            self.oids(&query.types),
            "parameter types of: {}",
            query.sql
        );
        let actual: Vec<u32> = statement
            .columns()
            .iter()
            .map(|c| c.type_().oid())
            .collect();
        let expected: Vec<u32> = self
            .oids(columns)
            .into_iter()
            .map(|oid| self.base(oid))
            .collect();
        assert_eq!(actual, expected, "column types of: {}", query.sql);
    }

    fn base(&mut self, mut oid: u32) -> u32 {
        loop {
            let (kind, base): (i8, u32) = self
                .client
                .query_one(
                    "SELECT typtype, typbasetype FROM pg_type WHERE oid = $1",
                    &[&oid],
                )
                .map(|row| (row.get(0), row.get(1)))
                .expect("type lookup must succeed");
            if kind != b'd' as i8 {
                return oid;
            }
            oid = base;
        }
    }

    fn bind_pair<P: surus::Params, R: Row>(
        &mut self,
        expected: surus::Query<P, R>,
        actual: surus::Query<P, R>,
        values: impl Bind<P> + Clone,
        lenient: bool,
    ) -> (Compiled, Compiled) {
        let expected = Compiled::typed(&expected.bind(values.clone()), false);
        let actual = Compiled::typed(&actual.bind(values), true);
        let columns = R::types();
        self.assert_shape(&expected, &columns, lenient);
        self.assert_shape(&actual, &columns, lenient);
        (expected, actual)
    }

    pub fn try_fetch(&mut self, query: &Compiled) -> Result<Vec<String>, postgres::Error> {
        self.try_fetch_as("row(q.*)::text", query)
    }

    pub fn try_fetch_as(
        &mut self,
        projection: &str,
        query: &Compiled,
    ) -> Result<Vec<String>, postgres::Error> {
        let wrapped = wrap(&query.sql, projection);
        let params: Vec<&(dyn ToSql + Sync)> = query.params.iter().map(|p| p.as_ref()).collect();
        self.exec("SAVEPOINT q");
        let rows = self
            .try_prepare(&wrapped, query)
            .and_then(|statement| self.client.query(&statement, &params))
            .map(|rows| rows.iter().map(|row| row.get(0)).collect());
        self.exec("ROLLBACK TO SAVEPOINT q");
        rows
    }

    pub fn fetch(&mut self, query: &Compiled) -> Vec<String> {
        self.try_fetch(query)
            .unwrap_or_else(|e| panic!("query failed: {e:?}\n{}", query.sql))
    }

    pub fn outcome(&mut self, query: &Compiled) -> Result<Vec<String>, SqlState> {
        self.try_fetch(query)
            .map_err(|e| e.code().cloned().expect("database error must carry a code"))
    }

    pub fn assert_same<P: surus::Params, R: Row>(
        &mut self,
        expected: surus::Query<P, R>,
        actual: surus::Query<P, R>,
        values: impl Bind<P> + Clone,
    ) {
        let (expected, actual) = self.bind_pair(expected, actual, values, false);
        let rows = self.fetch(&expected);
        require_rows(&rows, &expected.sql);
        assert_eq!(self.fetch(&actual), rows, "dsl sql: {}", actual.sql);
    }

    pub fn assert_same_named<P: surus::Params, R: Row>(
        &mut self,
        expected: surus::Query<P, R>,
        actual: surus::Query<P, R>,
        values: impl Bind<P> + Clone,
    ) {
        let (expected, actual) = self.bind_pair(expected, actual, values, false);
        let projection = "row_to_json(q)::text";
        let rows = self
            .try_fetch_as(projection, &expected)
            .unwrap_or_else(|e| panic!("query failed: {e:?}\n{}", expected.sql));
        require_rows(&rows, &expected.sql);
        let actual_rows = self
            .try_fetch_as(projection, &actual)
            .unwrap_or_else(|e| panic!("query failed: {e:?}\n{}", actual.sql));
        assert_eq!(actual_rows, rows, "dsl sql: {}", actual.sql);
    }

    pub fn assert_same_outcome<P: surus::Params, R: Row>(
        &mut self,
        expected: surus::Query<P, R>,
        actual: surus::Query<P, R>,
        values: impl Bind<P> + Clone,
    ) {
        let (expected, actual) = self.bind_pair(expected, actual, values, true);
        let outcome = self.outcome(&expected);
        if let Ok(rows) = &outcome {
            require_rows(rows, &expected.sql);
        }
        assert_eq!(self.outcome(&actual), outcome, "dsl sql: {}", actual.sql);
    }

    pub fn assert_same_error<P: surus::Params, R: Row>(
        &mut self,
        expected: surus::Query<P, R>,
        actual: surus::Query<P, R>,
        values: impl Bind<P> + Clone,
    ) {
        let (expected, actual) = self.bind_pair(expected, actual, values, true);
        let code = self
            .try_fetch(&expected)
            .expect_err("plain sql must fail")
            .code()
            .cloned();
        let error = match self.try_fetch(&actual) {
            Err(e) => e,
            Ok(rows) => panic!("dsl sql must fail: {}\nrows: {rows:?}", actual.sql),
        };
        assert_eq!(error.code(), code.as_ref(), "dsl sql: {}", actual.sql);
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

impl Drop for Case {
    fn drop(&mut self) {
        let _ = self.client.batch_execute("ROLLBACK");
    }
}
