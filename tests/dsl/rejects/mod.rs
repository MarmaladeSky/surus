//! Queries the type system must reject.
//!
//! Each function in `dsl.rs` builds a valid query, checked against plain SQL
//! like any other test, and wraps exactly one expression of it in
//! `swap!(valid, invalid)`, imported with `use super::swap;`. The normal build
//! uses `valid`. `rejections_fail_to_compile` builds the test crate again with
//! `--cfg reject`, which substitutes `invalid`, and requires for every case:
//!
//! - `valid` and `invalid` differ in at most 6 tokens;
//! - the substitution fails to compile inside that function, with error codes
//!   from the case's list in `CASES`;
//! - no compile error appears anywhere else.
#![allow(unexpected_cfgs)]

mod dsl;

use crate::support::{Case, rnd};
use std::path::Path;
use std::process::Command;

#[cfg(not(reject))]
#[allow(unused_macros)]
macro_rules! swap {
    ($valid:expr, $invalid:expr $(,)?) => {
        $valid
    };
}

#[cfg(reject)]
#[allow(unused_macros)]
macro_rules! swap {
    ($valid:expr, $invalid:expr $(,)?) => {
        $invalid
    };
}

#[allow(unused_imports)]
pub(crate) use swap;

const TYPE: &[&str] = &["E0061", "E0277", "E0308", "E0369", "E0599"];
const EXISTENCE: &[&str] = &[
    "E0061", "E0277", "E0308", "E0369", "E0425", "E0433", "E0599", "E0609",
];

const CASES: &[(&str, &[&str])] = &[
    ("column_existence", EXISTENCE),
    ("column_scope", TYPE),
    ("where_boolean", TYPE),
    ("having_boolean", TYPE),
    ("join_on_boolean", TYPE),
    ("update_where_boolean", TYPE),
    ("delete_where_boolean", TYPE),
    ("case_when_boolean", TYPE),
    ("filter_boolean", TYPE),
    ("comparison_types", TYPE),
    ("operator_operands", TYPE),
    ("function_arguments", TYPE),
    ("cast_validity", TYPE),
    ("parameter_mapping", TYPE),
    ("insert_value_type", TYPE),
    ("insert_not_null", TYPE),
    ("insert_arity", TYPE),
    ("update_assignment_type", TYPE),
    ("nullability_propagation", TYPE),
    ("result_row_type", TYPE),
    ("returning_type", TYPE),
    ("cte_column_type", TYPE),
    ("union_column_types", TYPE),
    ("union_arity", TYPE),
    ("scalar_subquery_shape", TYPE),
    ("in_subquery_type", TYPE),
];

const MAX_DIFF_TOKENS: usize = 6;
const DSL_PATH: &str = "tests/dsl/rejects/dsl.rs";

fn seed_users(case: &mut Case) {
    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{}');
         INSERT INTO users (name, email, group_id)
             SELECT '{}', NULL, id FROM groups
             UNION ALL SELECT '{}', '{}', id FROM groups
             UNION ALL SELECT '{}', '{}', NULL
             UNION ALL SELECT '{}', NULL, NULL;",
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));
}

fn seed_products(case: &mut Case) {
    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity) VALUES ('{}', {}, 1), ('{}', {}, 2)",
        rnd::text(),
        rnd::int(),
        rnd::text(),
        rnd::int(),
    ));
}

#[test]
fn column_existence() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT name FROM users ORDER BY name",
        dsl::column_existence,
    );
}

#[test]
fn column_scope() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT u.name FROM users u ORDER BY u.name",
        dsl::column_scope,
    );
}

#[test]
fn where_boolean() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT name FROM users WHERE email IS NULL ORDER BY name",
        dsl::where_boolean,
    );
}

#[test]
fn having_boolean() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT group_id, count(*) FROM users GROUP BY group_id HAVING count(*) > 1 ORDER BY group_id",
        dsl::having_boolean,
    );
}

#[test]
fn join_on_boolean() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT u.name, g.name FROM users u JOIN groups g ON g.id = u.group_id ORDER BY u.name",
        dsl::join_on_boolean,
    );
}

#[test]
fn update_where_boolean() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "UPDATE users SET email = name WHERE email IS NULL RETURNING name",
        dsl::update_where_boolean,
    );
}

#[test]
fn delete_where_boolean() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "DELETE FROM users WHERE email IS NULL RETURNING name",
        dsl::delete_where_boolean,
    );
}

#[test]
fn case_when_boolean() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT CASE WHEN email IS NULL THEN 'none' ELSE email END FROM users ORDER BY name",
        dsl::case_when_boolean,
    );
}

#[test]
fn filter_boolean() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT count(*) FILTER (WHERE email IS NOT NULL) FROM users",
        dsl::filter_boolean,
    );
}

#[test]
fn comparison_types() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT name FROM users WHERE id <> group_id ORDER BY name",
        dsl::comparison_types,
    );
}

#[test]
fn operator_operands() {
    let mut case = Case::new();
    case.exec(r#"INSERT INTO type_samples (v_int4, v_jsonb) VALUES (1, '{"a": 1}')"#);

    case.assert_same(
        "SELECT v_jsonb @> v_jsonb FROM type_samples",
        dsl::operator_operands,
    );
}

#[test]
fn function_arguments() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT lower(name) FROM users ORDER BY name",
        dsl::function_arguments,
    );
}

#[test]
fn cast_validity() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same("SELECT id::text FROM users ORDER BY id", dsl::cast_validity);
}

#[test]
fn parameter_mapping() {
    let mut case = Case::new();
    let id = 900_000_000 + (rnd::u64() % 100_000_000) as i64;
    case.exec(&format!(
        "INSERT INTO users (id, name) VALUES ({id}, '{}'), ({}, '{}')",
        rnd::text(),
        id + 1,
        rnd::text(),
    ));

    case.assert_same(&format!("SELECT name FROM users WHERE id = {id}"), || {
        dsl::parameter_mapping(id)
    });
}

#[test]
fn insert_value_type() {
    let mut case = Case::new();

    case.assert_same(
        "INSERT INTO products (name, price_cents, quantity) VALUES ('p', 10, 1) RETURNING name",
        dsl::insert_value_type,
    );
}

#[test]
fn insert_not_null() {
    let mut case = Case::new();

    case.assert_same(
        "INSERT INTO users (name) VALUES ('x') RETURNING name",
        dsl::insert_not_null,
    );
}

#[test]
fn insert_arity() {
    let mut case = Case::new();

    case.assert_same(
        "INSERT INTO users (name, email) VALUES ('x', NULL) RETURNING name",
        dsl::insert_arity,
    );
}

#[test]
fn update_assignment_type() {
    let mut case = Case::new();
    seed_products(&mut case);

    case.assert_same(
        "UPDATE products SET quantity = quantity + 1 RETURNING name, quantity",
        dsl::update_assignment_type,
    );
}

#[test]
fn nullability_propagation() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "INSERT INTO users (name) SELECT name || '!' FROM users RETURNING name",
        dsl::nullability_propagation,
    );
}

#[test]
fn result_row_type() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT id, name FROM users ORDER BY id",
        dsl::result_row_type,
    );
}

#[test]
fn returning_type() {
    let mut case = Case::new();

    case.assert_same(
        "INSERT INTO users (name) VALUES ('x') RETURNING id IS NOT NULL, name",
        dsl::returning_type,
    );
}

#[test]
fn cte_column_type() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "WITH t AS (SELECT id, name FROM users) SELECT name FROM t WHERE id > 0 ORDER BY name",
        dsl::cte_column_type,
    );
}

#[test]
fn union_column_types() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT id, name FROM users UNION SELECT id, name FROM groups ORDER BY name",
        dsl::union_column_types,
    );
}

#[test]
fn union_arity() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT id, name FROM users UNION SELECT id, name FROM groups ORDER BY name",
        dsl::union_arity,
    );
}

#[test]
fn scalar_subquery_shape() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT u.name, (SELECT g.name FROM groups g WHERE g.id = u.group_id) FROM users u ORDER BY u.name",
        dsl::scalar_subquery_shape,
    );
}

#[test]
fn in_subquery_type() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT name FROM users WHERE group_id IN (SELECT id FROM groups) ORDER BY name",
        dsl::in_subquery_type,
    );
}

#[test]
fn rejections_fail_to_compile() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = std::fs::read_to_string(root.join(DSL_PATH)).expect("dsl.rs must be readable");
    let tokens = tokenize(&source);

    let mut failures = Vec::new();
    let mut ranges = Vec::new();
    for (name, _) in CASES {
        match case_function(&tokens, name) {
            None => failures.push(format!("{name}: function not found")),
            Some((lines, body)) => {
                ranges.push((*name, lines));
                if let Err(problem) = check_swap(body) {
                    failures.push(format!("{name}: {problem}"));
                }
            }
        }
    }

    let errors = compile_rejected(root);
    let mut stray: Vec<&CompileError> = Vec::new();
    for error in &errors {
        let owner = ranges.iter().find(|(_, (first, last))| {
            error.path.ends_with(DSL_PATH) && (*first..=*last).contains(&error.line)
        });
        if owner.is_none() {
            stray.push(error);
        }
    }
    for (name, allowed) in CASES {
        let Some((_, (first, last))) = ranges.iter().find(|(n, _)| n == name) else {
            continue;
        };
        let own: Vec<&CompileError> = errors
            .iter()
            .filter(|e| e.path.ends_with(DSL_PATH) && (*first..=*last).contains(&e.line))
            .collect();
        if own.is_empty() {
            failures.push(format!("{name}: the invalid expression compiles"));
        }
        for error in own {
            if !error.code.as_deref().is_some_and(|c| allowed.contains(&c)) {
                failures.push(format!("{name}: disallowed error: {}", error.text));
            }
        }
    }
    for error in stray {
        failures.push(format!("error outside any case: {}", error.text));
    }

    assert!(
        failures.is_empty(),
        "rejection cases failed:\n{}",
        failures.join("\n")
    );
}

struct Token {
    text: String,
    line: usize,
}

fn tokenize(source: &str) -> Vec<Token> {
    let chars: Vec<char> = source.chars().collect();
    let at = |i: usize| chars.get(i).copied().unwrap_or('\0');
    let mut tokens = Vec::new();
    let (mut i, mut line) = (0, 1);
    while i < chars.len() {
        let (start, start_line) = (i, line);
        let c = chars[i];
        if c == '\n' {
            line += 1;
            i += 1;
            continue;
        }
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        if c == '/' && at(i + 1) == '/' {
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if c == '/' && at(i + 1) == '*' {
            i += 2;
            while i < chars.len() && !(chars[i] == '*' && at(i + 1) == '/') {
                line += usize::from(chars[i] == '\n');
                i += 1;
            }
            i += 2;
            continue;
        }
        let hashes = (i + 1..).take_while(|&j| at(j) == '#').count();
        if c == 'r' && at(i + 1 + hashes) == '"' {
            i += 2 + hashes;
            loop {
                if i >= chars.len() {
                    break;
                }
                if chars[i] == '"' && (1..=hashes).all(|k| at(i + k) == '#') {
                    i += 1 + hashes;
                    break;
                }
                line += usize::from(chars[i] == '\n');
                i += 1;
            }
        } else if c == '"' || (c == '\'' && (at(i + 1) == '\\' || at(i + 2) == '\'')) {
            i += 1;
            while i < chars.len() && chars[i] != c {
                if chars[i] == '\\' {
                    i += 1;
                }
                line += usize::from(at(i) == '\n');
                i += 1;
            }
            i += 1;
        } else if c.is_alphanumeric() || c == '_' || c == '\'' {
            i += 1;
            while at(i).is_alphanumeric() || at(i) == '_' {
                i += 1;
            }
        } else {
            i += 1;
        }
        tokens.push(Token {
            text: chars[start..i.min(chars.len())].iter().collect(),
            line: start_line,
        });
    }
    tokens
}

fn case_function<'a>(tokens: &'a [Token], name: &str) -> Option<((usize, usize), &'a [Token])> {
    let start = tokens
        .windows(2)
        .position(|w| w[0].text == "fn" && w[1].text == name)?;
    let open = start + tokens[start..].iter().position(|t| t.text == "{")?;
    let mut depth = 0;
    for (i, token) in tokens.iter().enumerate().skip(open) {
        match token.text.as_str() {
            "{" => depth += 1,
            "}" => {
                depth -= 1;
                if depth == 0 {
                    return Some(((tokens[start].line, token.line), &tokens[open..=i]));
                }
            }
            _ => {}
        }
    }
    None
}

fn check_swap(body: &[Token]) -> Result<(), String> {
    let starts: Vec<usize> = (0..body.len().saturating_sub(2))
        .filter(|&i| body[i].text == "swap" && body[i + 1].text == "!" && body[i + 2].text == "(")
        .collect();
    let [start] = starts[..] else {
        return Err(format!("expected one swap!(..), found {}", starts.len()));
    };

    let mut arguments: Vec<Vec<&str>> = vec![Vec::new()];
    let mut closers = vec![")"];
    for (i, token) in body.iter().enumerate().skip(start + 3) {
        let text = token.text.as_str();
        if closers.len() == 1 && text == ")" {
            break;
        }
        if closers.len() == 1 && text == "," {
            arguments.push(Vec::new());
            continue;
        }
        match text {
            "(" => closers.push(")"),
            "[" => closers.push("]"),
            "{" => closers.push("}"),
            "<" if i >= 2 && body[i - 1].text == ":" && body[i - 2].text == ":" => {
                closers.push(">")
            }
            _ if closers.last() == Some(&text) => {
                closers.pop();
            }
            _ => {}
        }
        arguments.last_mut().unwrap().push(text);
    }
    if arguments.last().is_some_and(|a| a.is_empty()) {
        arguments.pop();
    }
    let [valid, invalid] = &arguments[..] else {
        return Err(format!(
            "swap! takes 2 arguments, found {}",
            arguments.len()
        ));
    };

    let prefix = valid
        .iter()
        .zip(invalid)
        .take_while(|(a, b)| a == b)
        .count();
    let suffix = valid[prefix..]
        .iter()
        .rev()
        .zip(invalid[prefix..].iter().rev())
        .take_while(|(a, b)| a == b)
        .count();
    let changed = (valid.len() - prefix - suffix).max(invalid.len() - prefix - suffix);
    if changed > MAX_DIFF_TOKENS {
        return Err(format!(
            "swap! arguments differ in {changed} tokens, at most {MAX_DIFF_TOKENS} allowed"
        ));
    }
    Ok(())
}

struct CompileError {
    path: String,
    line: usize,
    code: Option<String>,
    text: String,
}

fn compile_rejected(root: &Path) -> Vec<CompileError> {
    let output = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()))
        .current_dir(root)
        .env(
            "CARGO_TARGET_DIR",
            Path::new(env!("CARGO_TARGET_TMPDIR")).join("rejects"),
        )
        .args([
            "rustc",
            "--offline",
            "--test",
            "dsl",
            "--profile",
            "check",
            "--message-format=short",
            "--",
            "--cfg",
            "reject",
        ])
        .output()
        .expect("cargo must run");
    let stderr = String::from_utf8_lossy(&output.stderr);
    let errors: Vec<CompileError> = stderr.lines().filter_map(parse_error).collect();
    assert!(
        output.status.success() || !errors.is_empty(),
        "rejection build failed without diagnostics:\n{stderr}"
    );
    errors
}

fn parse_error(line: &str) -> Option<CompileError> {
    let (location, message) = line.split_once(": error")?;
    let mut parts = location.rsplitn(3, ':');
    let _column: usize = parts.next()?.parse().ok()?;
    let line_number: usize = parts.next()?.parse().ok()?;
    let path = parts.next()?.to_string();
    let code = message
        .strip_prefix('[')
        .and_then(|rest| rest.split_once(']'))
        .map(|(code, _)| code.to_string());
    Some(CompileError {
        path,
        line: line_number,
        code,
        text: line.to_string(),
    })
}
