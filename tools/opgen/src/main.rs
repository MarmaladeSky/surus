use postgres::error::SqlState;
use postgres::{Client, NoTls};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

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

const PLACEHOLDER: &str = "    Query::plain(\"SELECT NULL WHERE false\")\n";

struct Signature {
    left: Option<&'static Concrete>,
    right: &'static Concrete,
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
            .map(|right| Signature { left: None, right })
            .collect();
    };
    let mut out = Vec::new();
    for l in candidates(left) {
        for r in &rights {
            if consistent(left, l, right, r) {
                out.push(Signature {
                    left: Some(l),
                    right: r,
                });
            }
        }
    }
    out
}

const UNRESOLVED: [SqlState; 2] = [SqlState::AMBIGUOUS_FUNCTION, SqlState::UNDEFINED_FUNCTION];

fn accepted(client: &mut Client, symbol: &str, s: &Signature) -> bool {
    let sql = match s.left {
        Some(l) => format!(
            "SELECT v_{} {symbol} v_{} FROM type_samples",
            l.ident, s.right.ident
        ),
        None => format!("SELECT {symbol} v_{} FROM type_samples", s.right.ident),
    };
    client.batch_execute("SAVEPOINT probe").unwrap();
    let result = client.simple_query(&sql);
    client.batch_execute("ROLLBACK TO SAVEPOINT probe").unwrap();
    match result {
        Ok(_) => true,
        Err(e) if e.code().is_some_and(|c| UNRESOLVED.contains(c)) => false,
        Err(e) => panic!("probe failed: {e}\n{sql}"),
    }
}

fn main() {
    let url = std::env::var("SURUS_TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());
    let mut client = Client::connect(&url, NoTls).expect("database must be running");
    let version: String = client.query_one("SHOW server_version", &[]).unwrap().get(0);
    let rows = client
        .query(
            "SELECT o.oprname, lt.typname, rt.typname
             FROM pg_operator o
             LEFT JOIN pg_type lt ON lt.oid = o.oprleft
             JOIN pg_type rt ON rt.oid = o.oprright
             WHERE o.oprnamespace = 'pg_catalog'::regnamespace
             ORDER BY 1, 2, 3",
            &[],
        )
        .unwrap();

    let tests = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/dsl");
    client.batch_execute("BEGIN").unwrap();
    for fixture in ["custom_types.sql", "type_samples.sql"] {
        let sql = fs::read_to_string(tests.join("schema").join(fixture)).unwrap();
        client.batch_execute(&sql).unwrap();
    }

    let names: BTreeMap<&str, &str> = NAMES.into_iter().collect();
    let mut by_operator: BTreeMap<&str, (String, Vec<Signature>)> = BTreeMap::new();
    let mut total = 0;
    for row in &rows {
        let symbol: String = row.get(0);
        let left: Option<String> = row.get(1);
        let right: String = row.get(2);
        let signatures: Vec<Signature> = resolve(left.as_deref(), &right)
            .into_iter()
            .filter(|s| accepted(&mut client, &symbol, s))
            .collect();
        if signatures.is_empty() {
            continue;
        }
        let name = names
            .get(symbol.as_str())
            .unwrap_or_else(|| panic!("no name for operator {symbol}"));
        total += signatures.len();
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

    let root = tests.join("operators");
    write_catalog(&root.join("catalog"), &version, &by_operator);
    write_dsl(&root.join("dsl"), &by_operator);
    println!(
        "{} operators, {} signatures, PostgreSQL {version}",
        by_operator.len(),
        total
    );
}

fn write_catalog(dir: &Path, version: &str, ops: &BTreeMap<&str, (String, Vec<Signature>)>) {
    let _ = fs::remove_dir_all(dir);
    fs::create_dir_all(dir).unwrap();
    let header =
        format!("// Generated by `cargo run -p opgen` from PostgreSQL {version}. Do not edit.\n");
    let mods: String = ops.keys().map(|name| format!("mod {name};\n")).collect();
    fs::write(dir.join("mod.rs"), format!("{header}\n{mods}")).unwrap();
    for (name, (symbol, signatures)) in ops {
        let pairs: String = signatures
            .iter()
            .map(|s| {
                let left = match s.left {
                    Some(left) => format!("Some(\"v_{}\")", left.ident),
                    None => "None".to_string(),
                };
                format!(
                    "            Pair {{\n                left: {left},\n                right: \"v_{}\",\n                dsl: dsl::{},\n            }},\n",
                    s.right.ident,
                    s.ident()
                )
            })
            .collect();
        let body = format!(
            "{header}\nuse super::super::dsl::{name} as dsl;\nuse super::super::{{Pair, run}};\n\n#[test]\nfn all() {{\n    run(\n        \"{symbol}\",\n        &[\n{pairs}        ],\n    );\n}}\n"
        );
        fs::write(dir.join(format!("{name}.rs")), body).unwrap();
    }
}

fn write_dsl(dir: &Path, ops: &BTreeMap<&str, (String, Vec<Signature>)>) {
    fs::create_dir_all(dir).unwrap();
    let mod_file = dir.join("mod.rs");
    let mut mods = fs::read_to_string(&mod_file).unwrap_or_default();
    for (name, (_, signatures)) in ops {
        let line = format!("pub mod {name};\n");
        if !mods.contains(&line) {
            mods.push_str(&line);
        }
        let file: PathBuf = dir.join(format!("{name}.rs"));
        let mut source = fs::read_to_string(&file)
            .unwrap_or_else(|_| "use crate::support::Query;\n".to_string());
        for ident in signatures.iter().map(Signature::ident) {
            if !source.contains(&format!("pub fn {ident}(")) {
                source.push_str(&format!(
                    "\npub fn {ident}() -> Query {{\n{PLACEHOLDER}}}\n"
                ));
            }
        }
        fs::write(file, source).unwrap();
    }
    fs::write(mod_file, mods).unwrap();
}
