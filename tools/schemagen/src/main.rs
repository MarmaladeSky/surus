//! Generates typed definitions from a PostgreSQL database:
//!
//! - `src/catalog.rs`: operators, casts and implicit coercions between the
//!   built-in types the DSL models, as resolved by the server;
//! - `schema/src/lib.rs`: the tables, views and user-defined types of the
//!   database, with the operators and casts involving those types.
//!
//! Run with `cargo run -p schemagen` against a database holding the schema
//! (for the tests, the fixtures in `tests/dsl/schema`, which the test suite
//! applies); the database URL is read from `SURUS_TEST_DATABASE_URL`.

use postgres::{Client, NoTls};
use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::Path;

/// Built-in types: SQL name, `pg_type.typname` and Rust type.
const BUILTIN: &[(&str, &str, &str)] = &[
    ("bool", "bool", "Bool"),
    ("int2", "int2", "Int2"),
    ("int4", "int4", "Int4"),
    ("int8", "int8", "Int8"),
    ("numeric", "numeric", "Numeric"),
    ("float4", "float4", "Float4"),
    ("float8", "float8", "Float8"),
    ("money", "money", "Money"),
    ("text", "text", "Text"),
    ("varchar", "varchar", "Varchar"),
    ("bpchar", "bpchar", "Bpchar"),
    ("\"char\"", "char", "Char"),
    ("date", "date", "Date"),
    ("time", "time", "Time"),
    ("timetz", "timetz", "Timetz"),
    ("timestamp", "timestamp", "Timestamp"),
    ("timestamptz", "timestamptz", "Timestamptz"),
    ("interval", "interval", "Interval"),
    ("bytea", "bytea", "Bytea"),
    ("uuid", "uuid", "Uuid"),
    ("json", "json", "Json"),
    ("jsonb", "jsonb", "Jsonb"),
    ("jsonpath", "jsonpath", "Jsonpath"),
    ("inet", "inet", "Inet"),
    ("cidr", "cidr", "Cidr"),
    ("macaddr", "macaddr", "Macaddr"),
    ("macaddr8", "macaddr8", "Macaddr8"),
    ("bit", "bit", "Bit"),
    ("varbit", "varbit", "Varbit"),
    ("point", "point", "Point"),
    ("line", "line", "Line"),
    ("lseg", "lseg", "Lseg"),
    ("box", "box", "PgBox"),
    ("circle", "circle", "Circle"),
    ("path", "path", "Path"),
    ("polygon", "polygon", "Polygon"),
    ("tsvector", "tsvector", "Tsvector"),
    ("tsquery", "tsquery", "Tsquery"),
    ("xml", "xml", "Xml"),
    ("int4[]", "_int4", "Array<Int4>"),
    ("text[]", "_text", "Array<Text>"),
    ("int4range", "int4range", "Range<Int4>"),
    ("tsrange", "tsrange", "Range<Timestamp>"),
    ("int4multirange", "int4multirange", "Multirange<Int4>"),
];

/// Binary operators and their marker types in `surus::op`.
const BINARY: &[(&str, &str)] = &[
    ("=", "Eq"),
    ("<>", "Ne"),
    ("<", "Lt"),
    ("<=", "Le"),
    (">", "Gt"),
    (">=", "Ge"),
    ("+", "Plus"),
    ("-", "Minus"),
    ("*", "Mul"),
    ("/", "Div"),
    ("%", "Rem"),
    ("^", "Pow"),
    ("&", "BitAnd"),
    ("|", "BitOr"),
    ("#", "Xor"),
    ("<<", "Shl"),
    (">>", "Shr"),
    ("||", "Concat"),
    ("~~", "Like"),
    ("~~*", "ILike"),
    ("!~~", "NotLike"),
    ("!~~*", "NotILike"),
    ("~", "Regex"),
    ("~*", "IRegex"),
    ("!~", "NotRegex"),
    ("!~*", "NotIRegex"),
    ("@>", "Contains"),
    ("<@", "ContainedBy"),
    (">>=", "ContainsOrEquals"),
    ("<<=", "ContainedByOrEquals"),
    ("&&", "Overlaps"),
    ("&<", "Overleft"),
    ("&>", "Overright"),
    ("&<|", "Overbelow"),
    ("|&>", "Overabove"),
    ("<<|", "StrictlyBelow"),
    ("|>>", "StrictlyAbove"),
    ("<^", "Below"),
    (">^", "Above"),
    ("-|-", "Adjacent"),
    ("<->", "Distance"),
    ("?#", "Intersects"),
    ("?-", "Horizontal"),
    ("?-|", "Perpendicular"),
    ("?||", "Parallel"),
    ("##", "ClosestPoint"),
    ("~=", "SameAs"),
    ("@@", "Matches"),
    ("@@@", "MatchesDeprecated"),
    ("^@", "StartsWith"),
    ("->", "Get"),
    ("->>", "GetText"),
    ("#>", "GetPath"),
    ("#>>", "GetPathText"),
    ("#-", "DeletePath"),
    ("?", "HasKey"),
    ("?|", "HasAnyKey"),
    ("?&", "HasAllKeys"),
    ("@?", "PathExists"),
    ("*=", "RecEq"),
    ("*<>", "RecNe"),
    ("*<", "RecLt"),
    ("*<=", "RecLe"),
    ("*>", "RecGt"),
    ("*>=", "RecGe"),
    ("~<~", "PatternLt"),
    ("~<=~", "PatternLe"),
    ("~>~", "PatternGt"),
    ("~>=~", "PatternGe"),
];

/// Prefix operators and their marker types in `surus::op`.
const PREFIX: &[(&str, &str)] = &[
    ("-", "Neg"),
    ("~", "Not"),
    ("!!", "Not"),
    ("+", "UnaryPlus"),
    ("@", "Abs"),
    ("|/", "Sqrt"),
    ("||/", "Cbrt"),
    ("@-@", "Length"),
    ("@@", "Center"),
    ("#", "Npoints"),
    ("?-", "IsHorizontal"),
    ("?|", "IsVertical"),
];

const KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "gen",
];

#[derive(Clone)]
struct Type {
    sql: String,
    typname: String,
    rust: String,
    user: bool,
}

enum UserKind {
    Enum,
    Domain(String),
    Composite(Vec<(String, String)>),
}

fn main() {
    let url = std::env::var("SURUS_TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());
    let mut client = Client::connect(&url, NoTls).expect("database must be running");
    let version: String = client.query_one("SHOW server_version", &[]).unwrap().get(0);
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

    let builtin: Vec<Type> = BUILTIN
        .iter()
        .map(|(sql, typname, rust)| Type {
            sql: sql.to_string(),
            typname: typname.to_string(),
            rust: rust.to_string(),
            user: false,
        })
        .collect();
    let mut catalog = format!(
        "//! Generated by `cargo run -p schemagen` from PostgreSQL {version}. Do not edit.\n\n"
    );
    catalog.push_str("use crate::types::*;\n\n");
    catalog.push_str(&environment(&mut client, &builtin, "crate"));
    catalog.push_str(&coercions(&mut client, &builtin));
    std::fs::write(root.join("src/catalog.rs"), catalog).unwrap();

    let (user, kinds) = user_types(&mut client);
    let all: Vec<Type> = builtin.iter().chain(&user).cloned().collect();
    let mut schema = format!(
        "//! Generated by `cargo run -p schemagen` from PostgreSQL {version}. Do not edit.\n\n\
         #![allow(non_upper_case_globals)]\n\nuse surus::*;\n\n"
    );
    for (ty, kind) in user.iter().zip(&kinds) {
        match kind {
            UserKind::Enum => {
                writeln!(schema, "surus::sql_enum!({} = {:?});", ty.rust, ty.sql).unwrap()
            }
            UserKind::Domain(base) => writeln!(
                schema,
                "surus::domain!({} = {:?}: {base});",
                ty.rust, ty.sql
            )
            .unwrap(),
            UserKind::Composite(fields) => {
                writeln!(
                    schema,
                    "surus::composite!({} = {:?}, {}Fields {{",
                    ty.rust, ty.sql, ty.rust
                )
                .unwrap();
                for (name, rust) in fields {
                    writeln!(schema, "    {}: {name:?} {rust};", field_name(name)).unwrap();
                }
                schema.push_str("});\n");
            }
        }
    }
    schema.push('\n');
    schema.push_str(&tables(&mut client, &all));
    schema.push_str(&environment(&mut client, &all, "surus"));
    std::fs::write(root.join("schema/src/lib.rs"), schema).unwrap();
    let formatted = std::process::Command::new("rustfmt")
        .args(["--edition", "2024"])
        .arg(root.join("src/catalog.rs"))
        .arg(root.join("schema/src/lib.rs"))
        .status();
    if !formatted.is_ok_and(|status| status.success()) {
        eprintln!("rustfmt failed; generated files are unformatted");
    }
    println!("generated from PostgreSQL {version}");
}

/// Maps a type name to its Rust type, including arrays of known types.
fn rust_type(types: &[Type], typname: &str) -> Option<String> {
    if let Some(ty) = types.iter().find(|t| t.typname == typname) {
        return Some(ty.rust.clone());
    }
    let element = typname.strip_prefix('_')?;
    types
        .iter()
        .find(|t| t.typname == element && !t.rust.contains('<'))
        .map(|t| format!("Array<{}>", t.rust))
}

fn camel_case(name: &str) -> String {
    name.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            let first = chars.next().unwrap().to_ascii_uppercase();
            first.to_string() + chars.as_str()
        })
        .collect()
}

fn field_name(name: &str) -> String {
    let snake: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();
    if KEYWORDS.contains(&snake.as_str()) {
        format!("r#{snake}")
    } else {
        snake
    }
}

/// Runs `probe` (a format string with `%s` placeholders) for each candidate
/// in a server-side loop and returns the result type names of those that
/// resolve.
fn probe(
    client: &mut Client,
    candidates: &[Vec<String>],
    probe: &str,
) -> Vec<(Vec<String>, String)> {
    client
        .batch_execute(
            "DROP TABLE IF EXISTS pg_temp.probe_candidates, pg_temp.probe_results;
             CREATE TEMP TABLE probe_candidates (args text[]);
             CREATE TEMP TABLE probe_results (args text[], result oid);",
        )
        .unwrap();
    let statement = client
        .prepare("INSERT INTO probe_candidates SELECT unnest($1::text[])::text[]")
        .unwrap();
    let encoded: Vec<String> = candidates
        .iter()
        .map(|args| {
            let quoted: Vec<String> = args
                .iter()
                .map(|a| format!("\"{}\"", a.replace('\\', "\\\\").replace('"', "\\\"")))
                .collect();
            format!("{{{}}}", quoted.join(","))
        })
        .collect();
    client.execute(&statement, &[&encoded]).unwrap();
    client
        .batch_execute(&format!(
            "DO $$
             DECLARE c record; t oid;
             BEGIN
               FOR c IN SELECT args FROM probe_candidates LOOP
                 BEGIN
                   EXECUTE format('SELECT pg_typeof({probe})::oid', VARIADIC c.args) INTO t;
                   INSERT INTO probe_results VALUES (c.args, t);
                 EXCEPTION WHEN OTHERS THEN NULL;
                 END;
               END LOOP;
             END $$;"
        ))
        .unwrap();
    client
        .query("SELECT r.args, t.typname::text FROM probe_results r JOIN pg_type t ON t.oid = r.result", &[])
        .unwrap()
        .iter()
        .map(|row| (row.get(0), row.get(1)))
        .collect()
}

/// Operators and casts among `types`, restricted to those involving a
/// user-defined type when any are present.
fn environment(client: &mut Client, types: &[Type], krate: &str) -> String {
    let relevant = |a: &Type, b: &Type| !types.iter().any(|t| t.user) || a.user || b.user;
    let symbols: Vec<String> = client
        .query("SELECT DISTINCT oprname::text FROM pg_operator WHERE oprleft <> 0 AND oprnamespace = 'pg_catalog'::regnamespace ORDER BY 1", &[])
        .unwrap()
        .iter()
        .map(|row| row.get(0))
        .collect();
    let mut out = String::new();

    let mut candidates = Vec::new();
    for symbol in symbols
        .iter()
        .filter(|s| BINARY.iter().any(|(b, _)| b == s))
    {
        for left in types {
            for right in types.iter().filter(|r| relevant(left, r)) {
                candidates.push(vec![left.sql.clone(), symbol.clone(), right.sql.clone()]);
            }
        }
    }
    let mut binary: BTreeMap<(String, String, String), String> = BTreeMap::new();
    for (args, result) in probe(client, &candidates, "NULL::%s %s NULL::%s") {
        let lookup = |sql: &str| {
            types
                .iter()
                .find(|t| t.sql == sql)
                .map(|t| t.rust.clone())
                .unwrap()
        };
        let marker = BINARY.iter().find(|(s, _)| *s == args[1]).unwrap().1;
        if let Some(result) = rust_type(types, &result) {
            binary.insert(
                (marker.to_string(), lookup(&args[0]), lookup(&args[2])),
                result,
            );
        }
    }
    writeln!(out, "{krate}::binary_operators! {{").unwrap();
    for ((marker, left, right), result) in &binary {
        writeln!(out, "    {marker}: {left}, {right} => {result};").unwrap();
    }
    out.push_str("}\n\n");

    let candidates: Vec<Vec<String>> = PREFIX
        .iter()
        .flat_map(|(symbol, _)| {
            types
                .iter()
                .filter(|t| relevant(t, t))
                .map(|t| vec![symbol.to_string(), t.sql.clone()])
        })
        .collect();
    let mut prefix: BTreeMap<(String, String), (String, String)> = BTreeMap::new();
    for (args, result) in probe(client, &candidates, "%s NULL::%s") {
        let operand = types
            .iter()
            .find(|t| t.sql == args[1])
            .unwrap()
            .rust
            .clone();
        let marker = PREFIX.iter().find(|(s, _)| *s == args[0]).unwrap().1;
        if let Some(result) = rust_type(types, &result) {
            prefix.insert((marker.to_string(), operand), (args[0].clone(), result));
        }
    }
    writeln!(out, "{krate}::prefix_operators! {{").unwrap();
    for ((marker, operand), (symbol, result)) in &prefix {
        writeln!(out, "    {marker} {symbol:?}: {operand} => {result};").unwrap();
    }
    out.push_str("}\n\n");

    let mut candidates = Vec::new();
    for from in types {
        for to in types
            .iter()
            .filter(|to| to.sql != from.sql && relevant(from, to))
        {
            candidates.push(vec![from.sql.clone(), to.sql.clone()]);
        }
    }
    let mut casts: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (args, _) in probe(client, &candidates, "CAST(NULL::%s AS %s)") {
        let rust = |sql: &str| types.iter().find(|t| t.sql == sql).unwrap().rust.clone();
        casts
            .entry(rust(&args[0]))
            .or_default()
            .push(rust(&args[1]));
    }
    writeln!(out, "{krate}::casts! {{").unwrap();
    for (from, to) in &mut casts {
        to.sort();
        writeln!(out, "    {from} => {};", to.join(", ")).unwrap();
    }
    out.push_str("}\n");
    out
}

/// Implicit casts between built-in types.
fn coercions(client: &mut Client, types: &[Type]) -> String {
    let rows = client
        .query(
            "SELECT s.typname::text, t.typname::text FROM pg_cast c
             JOIN pg_type s ON s.oid = c.castsource JOIN pg_type t ON t.oid = c.casttarget
             WHERE c.castcontext = 'i' AND c.castsource <> c.casttarget ORDER BY 1, 2",
            &[],
        )
        .unwrap();
    let mut coercions: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for row in &rows {
        let (from, to): (String, String) = (row.get(0), row.get(1));
        let from = types.iter().find(|t| t.typname == from);
        let to = types.iter().find(|t| t.typname == to);
        if let (Some(from), Some(to)) = (from, to) {
            coercions
                .entry(from.rust.clone())
                .or_default()
                .push(to.rust.clone());
        }
    }
    let mut out = String::from("\ncrate::coercions! {\n");
    for (from, to) in coercions {
        writeln!(out, "    {from} => {};", to.join(", ")).unwrap();
    }
    out.push_str("}\n");
    out
}

/// Enums, domains and composite types outside the system schemas.
fn user_types(client: &mut Client) -> (Vec<Type>, Vec<UserKind>) {
    let rows = client
        .query(
            "SELECT t.oid, n.nspname::text, t.typname::text, t.typtype::text, b.typname::text
             FROM pg_type t
             JOIN pg_namespace n ON n.oid = t.typnamespace
             LEFT JOIN pg_type b ON b.oid = t.typbasetype
             LEFT JOIN pg_class c ON c.oid = t.typrelid
             WHERE n.nspname NOT IN ('pg_catalog', 'information_schema') AND n.nspname NOT LIKE 'pg_toast%' AND n.nspname NOT LIKE 'pg_temp%'
               AND (t.typtype IN ('e', 'd') OR (t.typtype = 'c' AND c.relkind = 'c'))
             ORDER BY 3",
            &[],
        )
        .unwrap();
    let builtin: Vec<Type> = BUILTIN
        .iter()
        .map(|(sql, typname, rust)| Type {
            sql: sql.to_string(),
            typname: typname.to_string(),
            rust: rust.to_string(),
            user: false,
        })
        .collect();
    let mut types = Vec::new();
    let mut kinds = Vec::new();
    for row in &rows {
        let (oid, schema, name, kind): (postgres::types::Oid, String, String, String) =
            (row.get(0), row.get(1), row.get(2), row.get(3));
        let sql = if schema == "public" {
            name.clone()
        } else {
            format!("{schema}.{name}")
        };
        let kind = match kind.as_str() {
            "e" => UserKind::Enum,
            "d" => match rust_type(&builtin, &row.get::<_, String>(4)) {
                Some(base) => UserKind::Domain(base),
                None => continue,
            },
            _ => {
                let fields = client
                    .query(
                        "SELECT a.attname::text, t.typname::text FROM pg_attribute a JOIN pg_type t ON t.oid = a.atttypid
                         WHERE a.attrelid = (SELECT typrelid FROM pg_type WHERE oid = $1) AND a.attnum > 0 AND NOT a.attisdropped
                         ORDER BY a.attnum",
                        &[&oid],
                    )
                    .unwrap();
                let fields: Option<Vec<(String, String)>> = fields
                    .iter()
                    .map(|f| rust_type(&builtin, &f.get::<_, String>(1)).map(|t| (f.get(0), t)))
                    .collect();
                match fields {
                    Some(fields) => UserKind::Composite(fields),
                    None => continue,
                }
            }
        };
        types.push(Type {
            sql,
            typname: name.clone(),
            rust: camel_case(&name),
            user: true,
        });
        kinds.push(kind);
    }
    (types, kinds)
}

/// Tables and views with their columns and unique constraints, one module
/// per schema besides `public`.
fn tables(client: &mut Client, types: &[Type]) -> String {
    let relations = client
        .query(
            "SELECT c.oid, n.nspname::text, c.relname::text FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
             WHERE c.relkind IN ('r', 'v', 'p', 'm', 'f')
               AND n.nspname NOT IN ('pg_catalog', 'information_schema') AND n.nspname NOT LIKE 'pg_toast%' AND n.nspname NOT LIKE 'pg_temp%'
             ORDER BY n.nspname <> 'public', 2, 3",
            &[],
        )
        .unwrap();
    let mut modules: BTreeMap<String, String> = BTreeMap::new();
    for relation in &relations {
        let (oid, schema, name): (postgres::types::Oid, String, String) =
            (relation.get(0), relation.get(1), relation.get(2));
        let columns = client
            .query(
                "SELECT a.attname::text, t.typname::text, a.attnotnull FROM pg_attribute a JOIN pg_type t ON t.oid = a.atttypid
                 WHERE a.attrelid = $1 AND a.attnum > 0 AND NOT a.attisdropped ORDER BY a.attnum",
                &[&oid],
            )
            .unwrap();
        let out = modules.entry(schema.clone()).or_default();
        let value = field_name(&name);
        let rust = camel_case(&name);
        writeln!(
            out,
            "surus::table!({value}: {rust}({rust}Table) = {schema:?}.{name:?} {{"
        )
        .unwrap();
        for column in &columns {
            let (column, typname, not_null): (String, String, bool) =
                (column.get(0), column.get(1), column.get(2));
            match rust_type(types, &typname) {
                Some(ty) => {
                    let null = if not_null { "NotNull" } else { "Nullable" };
                    writeln!(out, "    {}: {column:?} {ty}, {null};", field_name(&column)).unwrap();
                }
                None => writeln!(out, "    // {column}: unsupported type {typname}").unwrap(),
            }
        }
        out.push_str("});\n");
        let constraints = client
            .query("SELECT conname::text FROM pg_constraint WHERE conrelid = $1 AND contype IN ('p', 'u', 'x') ORDER BY 1", &[&oid])
            .unwrap();
        for constraint in &constraints {
            let constraint: String = constraint.get(0);
            writeln!(
                out,
                "pub const {}: Constraint<{rust}Table> = Constraint::new({constraint:?});",
                field_name(&constraint)
            )
            .unwrap();
        }
        out.push('\n');
    }
    let mut out = modules.remove("public").unwrap_or_default();
    for (schema, body) in modules {
        writeln!(
            out,
            "pub mod {} {{\n    use surus::*;\n",
            field_name(&schema)
        )
        .unwrap();
        for line in body.lines() {
            if line.is_empty() {
                out.push('\n');
            } else {
                writeln!(out, "    {line}").unwrap();
            }
        }
        out.push_str("}\n\n");
    }
    out
}
