use postgres::error::SqlState;
use postgres::types::{Kind as TypeKind, Type};
use postgres::{Client, NoTls};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Scalar,
    Array(&'static str),
    Range(&'static str),
    Multirange(&'static str),
    Enum,
    Composite,
}

struct Concrete {
    typname: &'static str,
    ident: &'static str,
    kind: Kind,
}

impl Concrete {
    const fn scalar(typname: &'static str) -> Concrete {
        Concrete {
            typname,
            ident: typname,
            kind: Kind::Scalar,
        }
    }

    fn inner(&self) -> &'static str {
        match self.kind {
            Kind::Array(inner) | Kind::Range(inner) | Kind::Multirange(inner) => inner,
            _ => self.typname,
        }
    }
}

const CONCRETE: [Concrete; 46] = [
    Concrete::scalar("bool"),
    Concrete::scalar("int2"),
    Concrete::scalar("int4"),
    Concrete::scalar("int8"),
    Concrete::scalar("numeric"),
    Concrete::scalar("float4"),
    Concrete::scalar("float8"),
    Concrete::scalar("money"),
    Concrete::scalar("text"),
    Concrete::scalar("varchar"),
    Concrete::scalar("bpchar"),
    Concrete::scalar("char"),
    Concrete::scalar("date"),
    Concrete::scalar("time"),
    Concrete::scalar("timetz"),
    Concrete::scalar("timestamp"),
    Concrete::scalar("timestamptz"),
    Concrete::scalar("interval"),
    Concrete::scalar("bytea"),
    Concrete::scalar("uuid"),
    Concrete::scalar("json"),
    Concrete::scalar("jsonb"),
    Concrete::scalar("jsonpath"),
    Concrete::scalar("inet"),
    Concrete::scalar("cidr"),
    Concrete::scalar("macaddr"),
    Concrete::scalar("macaddr8"),
    Concrete::scalar("bit"),
    Concrete::scalar("varbit"),
    Concrete::scalar("point"),
    Concrete::scalar("line"),
    Concrete::scalar("lseg"),
    Concrete::scalar("box"),
    Concrete::scalar("circle"),
    Concrete::scalar("path"),
    Concrete::scalar("polygon"),
    Concrete::scalar("tsvector"),
    Concrete::scalar("tsquery"),
    Concrete::scalar("xml"),
    Concrete {
        typname: "_int4",
        ident: "int4_array",
        kind: Kind::Array("int4"),
    },
    Concrete {
        typname: "_text",
        ident: "text_array",
        kind: Kind::Array("text"),
    },
    Concrete {
        typname: "int4range",
        ident: "int4range",
        kind: Kind::Range("int4"),
    },
    Concrete {
        typname: "tsrange",
        ident: "tsrange",
        kind: Kind::Range("timestamp"),
    },
    Concrete {
        typname: "int4multirange",
        ident: "int4multirange",
        kind: Kind::Multirange("int4"),
    },
    Concrete {
        typname: "mood",
        ident: "mood",
        kind: Kind::Enum,
    },
    Concrete {
        typname: "price_tag",
        ident: "price_tag",
        kind: Kind::Composite,
    },
];

const NAMES: [(&str, &str); 74] = [
    ("!!", "fact"),
    ("!~", "not_regex"),
    ("!~*", "not_iregex"),
    ("!~~", "not_like"),
    ("!~~*", "not_ilike"),
    ("#", "xor"),
    ("##", "closest_point"),
    ("#-", "json_delete_path"),
    ("#>", "json_path"),
    ("#>>", "json_path_text"),
    ("%", "rem"),
    ("&", "bit_and"),
    ("&&", "overlaps"),
    ("&<", "overleft"),
    ("&<|", "overbelow"),
    ("&>", "overright"),
    ("*", "mul"),
    ("*<", "rec_lt"),
    ("*<=", "rec_le"),
    ("*<>", "rec_ne"),
    ("*=", "rec_eq"),
    ("*>", "rec_gt"),
    ("*>=", "rec_ge"),
    ("+", "plus"),
    ("-", "minus"),
    ("->", "json_get"),
    ("->>", "json_get_text"),
    ("-|-", "adjacent"),
    ("/", "div"),
    ("<", "lt"),
    ("<->", "distance"),
    ("<<", "shl"),
    ("<<=", "subnet_le"),
    ("<<|", "strictly_below"),
    ("<=", "le"),
    ("<>", "ne"),
    ("<@", "contained_by"),
    ("<^", "below"),
    ("=", "eq"),
    (">", "gt"),
    (">=", "ge"),
    (">>", "shr"),
    (">>=", "subnet_ge"),
    (">^", "above"),
    ("?", "json_exists"),
    ("?#", "intersects"),
    ("?&", "json_exists_all"),
    ("?-", "horizontal"),
    ("?-|", "perpendicular"),
    ("?|", "json_exists_any"),
    ("?||", "parallel"),
    ("@", "abs"),
    ("@-@", "length"),
    ("@>", "contains"),
    ("@?", "json_path_exists"),
    ("@@", "matches"),
    ("@@@", "matches_alt"),
    ("^", "pow"),
    ("^@", "starts_with"),
    ("|", "bit_or"),
    ("|&>", "overabove"),
    ("|/", "sqrt"),
    ("|>>", "strictly_above"),
    ("||", "concat"),
    ("||/", "cbrt"),
    ("~", "regex"),
    ("~*", "iregex"),
    ("~<=~", "pat_le"),
    ("~<~", "pat_lt"),
    ("~=", "same_as"),
    ("~>=~", "pat_ge"),
    ("~>~", "pat_gt"),
    ("~~", "like"),
    ("~~*", "ilike"),
];

struct Signature {
    left: Option<&'static Concrete>,
    right: &'static Concrete,
    result: String,
}

impl Signature {
    fn ident(&self) -> String {
        let name = match self.left {
            Some(left) => format!("{}_{}", left.ident, self.right.ident),
            None => self.right.ident.to_string(),
        };
        if name == "box" {
            "r#box".to_string()
        } else {
            name
        }
    }
}

fn is_container(typname: &str) -> bool {
    matches!(
        typname,
        "anyarray"
            | "anycompatiblearray"
            | "anyrange"
            | "anycompatiblerange"
            | "anymultirange"
            | "anycompatiblemultirange"
    )
}

fn is_polymorphic(typname: &str) -> bool {
    typname.starts_with("any") || typname == "record"
}

fn consistent(left: &str, l: &Concrete, right: &str, r: &Concrete) -> bool {
    if !is_polymorphic(left) || !is_polymorphic(right) {
        return true;
    }
    match (is_container(left), is_container(right)) {
        (true, true) => l.typname == r.typname || (l.kind != r.kind && l.inner() == r.inner()),
        (true, false) => l.inner() == r.typname,
        (false, true) => l.typname == r.inner(),
        (false, false) => l.typname == r.typname,
    }
}

fn candidates(typname: &str) -> Vec<&'static Concrete> {
    let matching = |pred: &dyn Fn(&Concrete) -> bool| CONCRETE.iter().filter(|c| pred(c)).collect();
    match typname {
        "anyarray" | "anycompatiblearray" => matching(&|c| matches!(c.kind, Kind::Array(_))),
        "anyrange" | "anycompatiblerange" => matching(&|c| matches!(c.kind, Kind::Range(_))),
        "anymultirange" | "anycompatiblemultirange" => {
            matching(&|c| matches!(c.kind, Kind::Multirange(_)))
        }
        "anyenum" => matching(&|c| c.kind == Kind::Enum),
        "record" => matching(&|c| c.kind == Kind::Composite),
        "anynonarray" | "anycompatiblenonarray" => matching(&|c| !matches!(c.kind, Kind::Array(_))),
        "anyelement" | "anycompatible" => CONCRETE.iter().collect(),
        concrete => matching(&|c| c.typname == concrete),
    }
}

fn resolve(left: Option<&str>, right: &str) -> Vec<Signature> {
    let rights = candidates(right);
    let Some(left) = left else {
        return rights
            .into_iter()
            .map(|right| Signature {
                left: None,
                right,
                result: String::new(),
            })
            .collect();
    };
    let mut out = Vec::new();
    for l in candidates(left) {
        for r in &rights {
            if consistent(left, l, right, r) {
                out.push(Signature {
                    left: Some(l),
                    right: r,
                    result: String::new(),
                });
            }
        }
    }
    out
}

const UNRESOLVED: [SqlState; 2] = [SqlState::AMBIGUOUS_FUNCTION, SqlState::UNDEFINED_FUNCTION];

fn marker(ty: &Type) -> String {
    match ty.kind() {
        TypeKind::Array(inner) => format!("t::Array<{}>", marker(inner)),
        TypeKind::Range(inner) => format!("t::Range<{}>", marker(inner)),
        TypeKind::Multirange(inner) => format!("t::Multirange<{}>", marker(inner)),
        TypeKind::Enum(_) | TypeKind::Composite(_) | TypeKind::Domain(_) => {
            format!("s::{}", type_ident(ty.name()))
        }
        _ => format!("t::{}", type_ident(ty.name())),
    }
}

fn accepted(client: &mut Client, symbol: &str, s: &Signature) -> Option<String> {
    let sql = match s.left {
        Some(l) => format!(
            "SELECT v_{} {symbol} v_{} FROM type_samples",
            l.ident, s.right.ident
        ),
        None => format!("SELECT {symbol} v_{} FROM type_samples", s.right.ident),
    };
    client.batch_execute("SAVEPOINT probe").unwrap();
    let result = client.prepare(&sql).and_then(|statement| {
        let result = format!("t::Nullable<{}>", marker(statement.columns()[0].type_()));
        client.query(&statement, &[]).map(|_| result)
    });
    client.batch_execute("ROLLBACK TO SAVEPOINT probe").unwrap();
    match result {
        Ok(result) => Some(result),
        Err(e) if e.code().is_some_and(|c| UNRESOLVED.contains(c)) => None,
        Err(e) => panic!("probe failed: {e}\n{sql}"),
    }
}
/// The DSL method for an operator; operators with a Rust counterpart keep
/// the names of their test files and are rendered as Rust operators.
fn method(symbol: &str, left: Option<&str>, right: &str) -> &'static str {
    let Some(l) = left else {
        return match symbol {
            "+" => "unary_plus",
            "-" => "neg",
            "~" => "bit_not",
            "!!" => "negate",
            "#" => "npoints",
            "?-" => "is_horizontal",
            "?|" => "is_vertical",
            "@" => "abs",
            "@-@" => "length",
            "@@" => "center",
            "|/" => "sqrt",
            "||/" => "cbrt",
            _ => panic!("no name for prefix operator {symbol}"),
        };
    };
    let bits = matches!(l, "int2" | "int4" | "int8" | "bit" | "varbit");
    let ranged = l.ends_with("range");
    let inet = matches!(l, "inet" | "cidr");
    match symbol {
        "+" if ranged => "union",
        "*" if ranged => "intersection",
        "-" if ranged => "difference",
        "-" if l == "jsonb" => "remove",
        "#" if bits => "bit_xor",
        "#" => "intersection",
        "<<" if inet => "subnet_lt",
        ">>" if inet => "subnet_gt",
        "<<" if !bits => "left_of",
        ">>" if !bits => "right_of",
        "&&" if l == "tsquery" => "ts_and",
        "||" if l == "tsquery" && right == "tsquery" => "ts_or",
        "<->" if l == "tsquery" => "followed_by",
        "?-" => "horizontally_aligned",
        "?|" if l == "point" => "vertically_aligned",
        "->" => "get",
        "->>" => "get_text",
        "#>" => "get_path",
        "#>>" => "get_path_text",
        "#-" => "remove_path",
        "?" => "has_key",
        "?|" if l == "point" => "vertically_aligned",
        "?|" => "has_any_key",
        "?&" => "has_all_keys",
        "@?" => "path_exists",
        "?||" => "is_parallel",
        "*=" => "image_eq",
        "*<>" => "image_ne",
        "*<" => "image_lt",
        "*<=" => "image_le",
        "*>" => "image_gt",
        "*>=" => "image_ge",
        "?-|" => "is_perpendicular",
        _ => {
            NAMES
                .iter()
                .find(|(s, _)| *s == symbol)
                .unwrap_or_else(|| panic!("no name for operator {symbol}"))
                .1
        }
    }
}

/// Rust operators for methods with the same symbol and meaning in Rust.
fn rust_operator(method: &str) -> Option<&'static str> {
    Some(match method {
        "plus" => "+",
        "minus" | "neg" => "-",
        "mul" => "*",
        "div" => "/",
        "rem" => "%",
        "bit_and" => "&",
        "bit_or" => "|",
        "bit_xor" => "^",
        "shl" => "<<",
        "shr" => ">>",
        "bit_not" => "!",
        _ => return None,
    })
}

/// The SQL spelling of an operator where it differs from the catalog symbol.
fn spelling(symbol: &str) -> &str {
    match symbol {
        "~~" => "LIKE",
        "~~*" => "ILIKE",
        "!~~" => "NOT LIKE",
        "!~~*" => "NOT ILIKE",
        _ => symbol,
    }
}

fn camel(snake: &str) -> String {
    type_ident(snake)
}

const STRINGS: [&str; 3] = ["text", "varchar", "bpchar"];

fn main() {
    let url = std::env::var("SURUS_TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());
    let mut client = Client::connect(&url, NoTls).expect("database must be running");
    let version: String = client.query_one("SHOW server_version", &[]).unwrap().get(0);
    let rows = client
        .query(
            "SELECT o.oprname, lt.typname, rt.typname, ot.typname
             FROM pg_operator o
             LEFT JOIN pg_type lt ON lt.oid = o.oprleft
             JOIN pg_type rt ON rt.oid = o.oprright
             JOIN pg_type ot ON ot.oid = o.oprresult
             WHERE o.oprnamespace = 'pg_catalog'::regnamespace
             ORDER BY 1, 2, 3",
            &[],
        )
        .unwrap();
    let scalars: Vec<(String, String)> = client
        .query(
            "SELECT typname, quote_ident(typname)
             FROM pg_type
             WHERE typnamespace = 'pg_catalog'::regnamespace
               AND typtype = 'b'
               AND typcategory <> 'A'
             ORDER BY typname",
            &[],
        )
        .unwrap()
        .iter()
        .map(|row| (row.get(0), row.get(1)))
        .collect();
    let ranges: Vec<(String, String, String)> = client
        .query(
            "SELECT s.typname, quote_ident(r.typname), quote_ident(m.typname)
             FROM pg_range g
             JOIN pg_type r ON r.oid = g.rngtypid
             JOIN pg_type m ON m.oid = g.rngmultitypid
             JOIN pg_type s ON s.oid = g.rngsubtype
             WHERE r.typnamespace = 'pg_catalog'::regnamespace
             ORDER BY s.typname",
            &[],
        )
        .unwrap()
        .iter()
        .map(|row| (row.get(0), row.get(1), row.get(2)))
        .collect();
    let casts: Vec<(String, String, i8)> = client
        .query(
            "SELECT s.typname, t.typname, c.castcontext
             FROM pg_cast c
             JOIN pg_type s ON s.oid = c.castsource
             JOIN pg_type t ON t.oid = c.casttarget
             WHERE c.castsource <> c.casttarget
             ORDER BY 1, 2",
            &[],
        )
        .unwrap()
        .iter()
        .map(|row| (row.get(0), row.get(1), row.get(2)))
        .collect();

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let tests = root.join("tests/dsl");
    let known: BTreeSet<&str> = scalars.iter().map(|(t, _)| t.as_str()).collect();
    write_types(
        &root.join("src/types/catalog.rs"),
        &version,
        &scalars,
        &ranges,
    );
    let mut operators: Vec<Operator> = rows
        .iter()
        .map(|row| Operator {
            symbol: row.get(0),
            left: row.get(1),
            right: row.get(2),
            result: row.get(3),
        })
        .collect();
    operators.extend(coerced(&mut client, &operators));
    write_ops(
        &root.join("src/ops/catalog.rs"),
        &version,
        &operators,
        &known,
    );
    write_casts(&root.join("src/types/casts.rs"), &version, &casts, &known);

    client.batch_execute("BEGIN").unwrap();
    for fixture in ["custom_types.sql", "type_samples.sql"] {
        let sql = fs::read_to_string(tests.join("schema").join(fixture)).unwrap();
        client.batch_execute(&sql).unwrap();
    }
    let mut by_operator: BTreeMap<&str, (String, Vec<Signature>)> = BTreeMap::new();
    for row in &rows {
        let symbol: String = row.get(0);
        let left: Option<String> = row.get(1);
        let right: String = row.get(2);
        let signatures: Vec<Signature> = resolve(left.as_deref(), &right)
            .into_iter()
            .filter_map(|mut s| {
                s.result = accepted(&mut client, &symbol, &s)?;
                Some(s)
            })
            .collect();
        if signatures.is_empty() {
            continue;
        }
        let name = NAMES
            .iter()
            .find(|(s, _)| *s == symbol)
            .unwrap_or_else(|| panic!("no name for operator {symbol}"))
            .1;
        by_operator
            .entry(name)
            .or_insert_with(|| (symbol.clone(), Vec::new()))
            .1
            .extend(signatures);
    }
    client.batch_execute("ROLLBACK").unwrap();
    for (_, signatures) in by_operator.values_mut() {
        signatures.sort_by_key(Signature::ident);
        signatures.dedup_by_key(|s| s.ident());
    }
    write_dsl(&tests.join("operators/dsl"), &by_operator);
    println!(
        "{} operators, {} types, PostgreSQL {version}",
        rows.len(),
        scalars.len()
    );
}

fn header(version: &str) -> String {
    format!("// Generated by `cargo run -p opgen` from PostgreSQL {version}. Do not edit.\n")
}

const PRELUDE: [&str; 1] = ["Box"];

fn type_ident(typname: &str) -> String {
    let ident: String = typname
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            chars
                .next()
                .map(|first| first.to_ascii_uppercase().to_string() + chars.as_str())
                .unwrap_or_default()
        })
        .collect();
    if PRELUDE.contains(&ident.as_str()) {
        format!("Pg{ident}")
    } else {
        ident
    }
}

/// The Rust marker of a catalog type, if the DSL knows it.
fn rust_type(typname: &str, known: &BTreeSet<&str>) -> Option<String> {
    match typname.strip_prefix('_') {
        Some(element) if known.contains(element) => Some(format!("Array<{}>", type_ident(element))),
        None if known.contains(typname) => Some(type_ident(typname)),
        _ => None,
    }
}

fn write_types(
    file: &Path,
    version: &str,
    scalars: &[(String, String)],
    ranges: &[(String, String, String)],
) {
    let mut out = format!("{}\nuse super::*;\n\nscalars! {{\n", header(version));
    for (typname, sql) in scalars {
        out += &format!("    {} {sql:?}\n", type_ident(typname));
    }
    out += "}\n\nranges! {\n";
    for (subtype, range, multirange) in ranges {
        out += &format!("    {} {range:?} {multirange:?}\n", type_ident(subtype));
    }
    out += "}\n";
    write(file, out);
}

fn write_ops(file: &Path, version: &str, operators: &[Operator], known: &BTreeSet<&str>) {
    let mut groups: BTreeMap<(bool, String, String), Vec<String>> = BTreeMap::new();
    for o in operators {
        let symbol = o.symbol.as_str();
        let left = o.left.as_deref();
        let types = [left.unwrap_or("bool"), &o.right, &o.result];
        let Some(rust) = types
            .iter()
            .map(|t| rust_type(t, known))
            .collect::<Option<Vec<String>>>()
        else {
            continue;
        };
        let operator = camel(method(symbol, left, &o.right));
        let entry = groups
            .entry((left.is_none(), operator, spelling(symbol).to_string()))
            .or_default();
        entry.push(match left {
            Some(_) => format!("{}, {} => {}", rust[0], rust[1], rust[2]),
            None => format!("{} => {}", rust[1], rust[2]),
        });
    }
    // text || anynonarray and anynonarray || text
    let concat = groups
        .entry((false, "Concat".to_string(), "||".to_string()))
        .or_default();
    for typname in known.iter().filter(|t| **t != "text") {
        let ident = type_ident(typname);
        concat.push(format!("Text, {ident} => Text"));
        concat.push(format!("{ident}, Text => Text"));
    }
    let mut out = format!("{}\nuse super::*;\n", header(version));
    for ((prefix, operator, symbol), impls) in groups {
        let kind = if prefix { "prefix" } else { "binary" };
        out += &format!("\n{kind}! {{ {operator} {symbol:?}:\n");
        for i in impls {
            out += &format!("    {i};\n");
        }
        out += "}\n";
    }
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    write(file, out);
}

fn write_casts(file: &Path, version: &str, casts: &[(String, String, i8)], known: &BTreeSet<&str>) {
    let mut explicit = BTreeSet::new();
    let mut implicit = BTreeSet::new();
    let mut assignment = BTreeSet::new();
    for (source, target, context) in casts {
        let (Some(s), Some(t)) = (rust_type(source, known), rust_type(target, known)) else {
            continue;
        };
        explicit.insert((s.clone(), t.clone()));
        if *context != b'e' as i8 {
            assignment.insert((s.clone(), t.clone()));
        }
        if *context == b'i' as i8 {
            implicit.insert((s, t));
        }
    }
    // I/O conversion casts to and from the string types
    for typname in known {
        for string in STRINGS.iter().filter(|s| *s != typname) {
            explicit.insert((type_ident(typname), type_ident(string)));
            explicit.insert((type_ident(string), type_ident(typname)));
        }
    }
    let list = |pairs: BTreeSet<(String, String)>| -> String {
        pairs
            .iter()
            .map(|(s, t)| format!("    {s} => {t};\n"))
            .collect()
    };
    let out = format!(
        "{}\nuse super::*;\n\nmarkers! {{ Cast:\n{}}}\n\nmarkers! {{ Coerce:\n{}}}\n\nmarkers! {{ Assign:\n{}}}\n",
        header(version),
        list(explicit),
        list(implicit),
        list(assignment),
    );
    write(file, out);
}

fn write_dsl(dir: &Path, ops: &BTreeMap<&str, (String, Vec<Signature>)>) {
    for (name, (symbol, signatures)) in ops {
        let schema = signatures.iter().any(|s| s.result.contains("s::"));
        let mut source = format!(
            "#![allow(clippy::eq_op)]\n\nuse crate::schema::type_samples;\n{}use surus::Query;\nuse surus::prelude::*;\nuse surus::types as t;\n",
            if schema {
                "use crate::schema as s;\n"
            } else {
                ""
            }
        );
        for s in signatures {
            let right = format!("s.v_{}", s.right.ident);
            let method = method(symbol, s.left.map(|l| l.typname), s.right.typname);
            let expression = match (s.left, rust_operator(method)) {
                (Some(l), Some(op)) => format!("s.v_{} {op} {right}", l.ident),
                (Some(l), None) => format!("s.v_{}.{method}({right})", l.ident),
                (None, Some(op)) => format!("{op}{right}"),
                (None, None) => format!("{right}.{method}()"),
            };
            source += &format!(
                "\npub fn {}() -> Query<(), ({},)> {{\n    type_samples.select(|s| {expression}).into()\n}}\n",
                s.ident(),
                s.result
            );
        }
        write(&dir.join(format!("{name}.rs")), source);
    }
}

/// Operators PostgreSQL resolves through implicit casts between related
/// types, such as `numeric / int4`, probed on the server.
fn coerced(client: &mut Client, exact: &[Operator]) -> Vec<Operator> {
    const FAMILIES: [&[&str]; 3] = [
        &["int2", "int4", "int8", "numeric", "float4", "float8"],
        &["text", "varchar", "bpchar", "name"],
        &["date", "timestamp", "timestamptz"],
    ];
    const SYMBOLS: [&str; 20] = [
        "+", "-", "*", "/", "%", "^", "=", "<>", "<", "<=", ">", ">=", "~~", "~~*", "!~~", "!~~*",
        "~", "~*", "!~", "!~*",
    ];
    let mut found = Vec::new();
    for family in FAMILIES {
        for left in family {
            for right in family {
                for symbol in SYMBOLS {
                    let defined = exact.iter().any(|o| {
                        o.symbol == symbol && o.left.as_deref() == Some(*left) && o.right == *right
                    });
                    if defined {
                        continue;
                    }
                    let sql = format!("SELECT NULL::{left} {symbol} NULL::{right}");
                    if let Ok(statement) = client.prepare(&sql) {
                        found.push(Operator {
                            symbol: symbol.to_string(),
                            left: Some(left.to_string()),
                            right: right.to_string(),
                            result: statement.columns()[0].type_().name().to_string(),
                        });
                    }
                }
            }
        }
    }
    found
}

struct Operator {
    symbol: String,
    left: Option<String>,
    right: String,
    result: String,
}

/// Writes a generated Rust file, formatted.
fn write(file: &Path, source: String) {
    fs::write(file, source).unwrap();
    let status = std::process::Command::new("rustfmt")
        .args(["--edition", "2024"])
        .arg(file)
        .status()
        .expect("rustfmt must run");
    assert!(status.success(), "rustfmt failed on {}", file.display());
}
