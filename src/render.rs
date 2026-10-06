//! Serialization of the syntax tree into PostgreSQL SQL with parameters.

use crate::dml::{InsertSource, OnConflict};
use crate::expr::{Node, Param, Value, Window};
use crate::query::{CteDef, FromItem, FromKind, Item, Join, SelectNode, Stmt};
use std::marker::PhantomData;
use std::rc::Rc;

/// SQL text with its parameters; `R` is the result row type.
#[derive(Debug)]
pub struct Compiled<R> {
    pub sql: String,
    /// Parameters `$1`, `$2`, ... in order.
    pub params: Vec<Param>,
    _row: PhantomData<fn() -> R>,
}

pub(crate) fn compile<R>(stmt: Stmt) -> Compiled<R> {
    let mut renderer = Renderer::default();
    renderer.stmt(&stmt);
    let mut sql = String::new();
    if !renderer.ctes.is_empty() {
        sql.push_str(if renderer.recursive {
            "WITH RECURSIVE "
        } else {
            "WITH "
        });
        let ctes: Vec<String> = renderer.ctes.into_iter().map(|(_, text)| text).collect();
        sql.push_str(&ctes.join(", "));
        sql.push(' ');
    }
    sql.push_str(&renderer.out);
    let params = renderer.params.iter().map(|p| Param::clone(p)).collect();
    Compiled {
        sql,
        params,
        _row: PhantomData,
    }
}

#[derive(Default)]
struct Renderer {
    out: String,
    params: Vec<Rc<Param>>,
    ctes: Vec<(Rc<CteDef>, String)>,
    visiting: Vec<*const CteDef>,
    recursive: bool,
}

const RESERVED: [&str; 101] = [
    "all",
    "analyse",
    "analyze",
    "and",
    "any",
    "array",
    "as",
    "asc",
    "asymmetric",
    "authorization",
    "binary",
    "both",
    "case",
    "cast",
    "check",
    "collate",
    "collation",
    "column",
    "concurrently",
    "constraint",
    "create",
    "cross",
    "current_catalog",
    "current_date",
    "current_role",
    "current_schema",
    "current_time",
    "current_timestamp",
    "current_user",
    "default",
    "deferrable",
    "desc",
    "distinct",
    "do",
    "else",
    "end",
    "except",
    "false",
    "fetch",
    "for",
    "foreign",
    "freeze",
    "from",
    "full",
    "grant",
    "group",
    "having",
    "ilike",
    "in",
    "initially",
    "inner",
    "intersect",
    "into",
    "is",
    "isnull",
    "join",
    "lateral",
    "leading",
    "left",
    "like",
    "limit",
    "localtime",
    "localtimestamp",
    "natural",
    "not",
    "notnull",
    "null",
    "offset",
    "on",
    "only",
    "or",
    "order",
    "outer",
    "overlaps",
    "placing",
    "primary",
    "references",
    "returning",
    "right",
    "select",
    "session_user",
    "similar",
    "some",
    "symmetric",
    "system_user",
    "table",
    "tablesample",
    "then",
    "to",
    "trailing",
    "true",
    "union",
    "unique",
    "user",
    "using",
    "variadic",
    "verbose",
    "when",
    "where",
    "window",
    "with",
];

impl Renderer {
    fn push(&mut self, text: &str) {
        self.out.push_str(text);
    }

    fn ident(&mut self, name: &str) {
        let plain = name.starts_with(|c: char| c.is_ascii_lowercase() || c == '_')
            && name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
            && !RESERVED.contains(&name);
        if plain {
            self.push(name);
        } else {
            self.out.push('"');
            self.push(&name.replace('"', "\"\""));
            self.out.push('"');
        }
    }

    fn list<T>(&mut self, items: &[T], separator: &str, mut each: impl FnMut(&mut Self, &T)) {
        for (i, item) in items.iter().enumerate() {
            if i > 0 {
                self.push(separator);
            }
            each(self, item);
        }
    }

    fn nodes(&mut self, nodes: &[Node]) {
        self.list(nodes, ", ", Self::node);
    }

    fn param(&mut self, param: &Rc<Param>) {
        let index = match self.params.iter().position(|p| Rc::ptr_eq(p, param)) {
            Some(index) => index,
            None => {
                self.params.push(param.clone());
                self.params.len() - 1
            }
        };
        self.push(&format!("${}", index + 1));
        if param.value != Value::Placeholder {
            self.push("::");
            self.push(&param.sql_type);
        }
    }

    fn node(&mut self, node: &Node) {
        match node {
            Node::Column(qualifier, name) => {
                if !qualifier.is_empty() {
                    self.ident(qualifier);
                    self.push(".");
                }
                // `*`, or an output column position in a set operation's `ORDER BY`.
                if *name == "*"
                    || qualifier.is_empty() && name.starts_with(|c: char| c.is_ascii_digit())
                {
                    self.push(name);
                } else {
                    self.ident(name);
                }
            }
            Node::Param(param) => self.param(param),
            Node::Literal(text, sql_type) => {
                self.push(&format!("'{}'", text.replace('\'', "''")));
                if !sql_type.is_empty() {
                    self.push("::");
                    self.push(sql_type);
                }
            }
            Node::Type(name) => self.push(name),
            Node::Keyword(keyword) => self.push(keyword),
            Node::Ident(name) => self.ident(name),
            Node::Binary(op, operands) => {
                self.push("(");
                self.node(&operands[0]);
                self.push(&format!(" {op} "));
                self.node(&operands[1]);
                self.push(")");
            }
            Node::Prefix(op, operand) => {
                self.push(&format!("({op} "));
                self.node(operand);
                self.push(")");
            }
            Node::Postfix(operand, op) => {
                self.push("(");
                self.node(operand);
                self.push(&format!(" {op})"));
            }
            Node::Call(call) => {
                self.push(call.name);
                self.push("(");
                if call.distinct {
                    self.push("DISTINCT ");
                }
                self.nodes(&call.args);
                if !call.order.is_empty() {
                    self.push(" ORDER BY ");
                    self.nodes(&call.order);
                }
                self.push(")");
                if !call.within_group.is_empty() {
                    self.push(" WITHIN GROUP (ORDER BY ");
                    self.nodes(&call.within_group);
                    self.push(")");
                }
                if let Some(filter) = &call.filter {
                    self.push(" FILTER (WHERE ");
                    self.node(filter);
                    self.push(")");
                }
                if let Some(window) = &call.over {
                    self.push(" OVER ");
                    self.over(window);
                }
            }
            Node::Cast(operand, sql_type) => {
                self.push("CAST(");
                self.node(operand);
                self.push(&format!(" AS {sql_type})"));
            }
            Node::Field(operand, field) => {
                self.push("(");
                self.node(operand);
                self.push(").");
                if *field == "*" {
                    self.push("*");
                } else {
                    self.ident(field);
                }
            }
            Node::Index(operand, index) => {
                self.push("(");
                self.node(operand);
                self.push(")[");
                self.list(index, "", Self::node);
                self.push("]");
            }
            Node::Array(items) => {
                self.push("ARRAY[");
                self.nodes(items);
                self.push("]");
            }
            Node::List(items) => {
                self.push("(");
                self.nodes(items);
                self.push(")");
            }
            Node::Seq(parts) => self.list(parts, " ", Self::node),
            Node::Subquery(stmt) => {
                self.push("(");
                self.stmt(stmt);
                self.push(")");
            }
        }
    }

    fn over(&mut self, window: &Window) {
        if let (Some(name), true) = (
            window.base,
            window.partition.is_empty() && window.order.is_empty() && window.frame.is_empty(),
        ) {
            self.ident(name);
            return;
        }
        self.push("(");
        self.window(window);
        self.push(")");
    }

    fn window(&mut self, window: &Window) {
        let mut parts = 0;
        let mut part = |this: &mut Self| {
            if parts > 0 {
                this.push(" ");
            }
            parts += 1;
        };
        if let Some(name) = window.base {
            part(self);
            self.ident(name);
        }
        if !window.partition.is_empty() {
            part(self);
            self.push("PARTITION BY ");
            self.nodes(&window.partition);
        }
        if !window.order.is_empty() {
            part(self);
            self.push("ORDER BY ");
            self.nodes(&window.order);
        }
        if !window.frame.is_empty() {
            part(self);
            self.list(&window.frame, " ", Self::node);
        }
    }

    fn items(&mut self, items: &[Item]) {
        self.list(items, ", ", |this, item| {
            this.node(&item.node);
            if let Some(alias) = item.alias {
                this.push(" AS ");
                this.ident(alias);
            }
        });
    }

    fn returning(&mut self, items: &[Item]) {
        if !items.is_empty() {
            self.push(" RETURNING ");
            self.items(items);
        }
    }

    fn filter(&mut self, filter: &Option<Node>) {
        if let Some(condition) = filter {
            self.push(" WHERE ");
            self.node(condition);
        }
    }

    fn cte(&mut self, def: &Rc<CteDef>) {
        let ptr = Rc::as_ptr(def);
        if self.visiting.contains(&ptr) || self.ctes.iter().any(|(d, _)| Rc::ptr_eq(d, def)) {
            return;
        }
        self.visiting.push(ptr);
        let outer = std::mem::take(&mut self.out);
        self.ident(def.name);
        self.push("(");
        self.list(&def.columns, ", ", |this, c| this.ident(c));
        self.push(") AS ");
        match def.materialized {
            Some(true) => self.push("MATERIALIZED "),
            Some(false) => self.push("NOT MATERIALIZED "),
            None => {}
        }
        self.push("(");
        self.stmt(&def.body);
        self.push(")");
        let text = std::mem::replace(&mut self.out, outer);
        self.visiting.pop();
        self.recursive |= def.recursive;
        self.ctes.push((def.clone(), text));
    }

    fn from_item(&mut self, item: &FromItem) {
        if item.lateral {
            self.push("LATERAL ");
        }
        match &item.kind {
            FromKind::Table(schema, name) => {
                self.ident(schema);
                self.push(".");
                self.ident(name);
            }
            FromKind::Query(stmt) => {
                self.push("(");
                self.stmt(stmt);
                self.push(")");
            }
            FromKind::Function(call) => self.node(call),
            FromKind::Cte(def) => {
                self.cte(def);
                self.ident(def.name);
            }
            FromKind::CteSelf(name) => self.ident(name),
        }
        if let Some(alias) = item.alias {
            self.push(" AS ");
            self.ident(alias);
            if !item.columns.is_empty() {
                self.push("(");
                self.list(&item.columns, ", ", |this, c| this.ident(c));
                self.push(")");
            }
        }
        if let Some(sample) = &item.sample {
            self.push(" ");
            self.node(sample);
        }
    }

    fn joins(&mut self, joins: &[Join]) {
        for join in joins {
            match join.kind {
                "" => {}
                "," => self.push(", "),
                kind => {
                    self.push(" ");
                    self.push(kind);
                    self.push(" ");
                }
            }
            self.from_item(&join.item);
            if let Some(condition) = &join.condition {
                self.push(" ");
                self.node(condition);
            }
        }
    }

    fn select(&mut self, select: &SelectNode) {
        self.push("SELECT ");
        match &select.distinct {
            Some(keys) if keys.is_empty() => self.push("DISTINCT "),
            Some(keys) => {
                self.push("DISTINCT ON (");
                self.nodes(keys);
                self.push(") ");
            }
            None => {}
        }
        self.items(&select.items);
        if !select.from.is_empty() {
            self.push(" FROM ");
            self.joins(&select.from);
        }
        self.filter(&select.filter);
        if !select.group.is_empty() {
            self.push(" GROUP BY ");
            self.nodes(&select.group);
        }
        if let Some(having) = &select.having {
            self.push(" HAVING ");
            self.node(having);
        }
        if !select.windows.is_empty() {
            self.push(" WINDOW ");
            self.list(&select.windows, ", ", |this, (name, window)| {
                this.ident(name);
                this.push(" AS (");
                this.window(window);
                this.push(")");
            });
        }
        self.order_by(&select.order);
        if let Some(limit) = &select.limit {
            self.push(" LIMIT ");
            self.node(limit);
        }
        if let Some(offset) = &select.offset {
            self.push(" OFFSET ");
            self.node(offset);
        }
        if let Some((count, with_ties)) = &select.fetch {
            self.push(" FETCH FIRST (");
            self.node(count);
            self.push(if *with_ties {
                ") ROWS WITH TIES"
            } else {
                ") ROWS ONLY"
            });
        }
        for lock in &select.locks {
            self.push(" ");
            self.list(lock, " ", Self::node);
        }
    }

    fn order_by(&mut self, keys: &[Node]) {
        if !keys.is_empty() {
            self.push(" ORDER BY ");
            self.nodes(keys);
        }
    }

    /// A set operation operand, parenthesized when needed.
    fn operand(&mut self, stmt: &Stmt) {
        let plain = match stmt {
            Stmt::Select(s) => {
                s.order.is_empty()
                    && s.limit.is_none()
                    && s.offset.is_none()
                    && s.fetch.is_none()
                    && s.locks.is_empty()
            }
            Stmt::Values(_) => true,
            _ => false,
        };
        if plain {
            self.stmt(stmt);
        } else {
            self.push("(");
            self.stmt(stmt);
            self.push(")");
        }
    }

    fn assignments(&mut self, set: &[Node]) {
        self.push(" SET ");
        self.nodes(set);
    }

    fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Select(select) => self.select(select),
            Stmt::SetOp(op) => {
                self.operand(&op.left);
                self.push(&format!(" {} ", op.op));
                self.operand(&op.right);
                self.order_by(&op.order);
            }
            Stmt::Values(rows) => {
                self.push("VALUES ");
                self.list(rows, ", ", |this, row| {
                    this.push("(");
                    this.nodes(row);
                    this.push(")");
                });
            }
            Stmt::Insert(insert) => {
                self.push("INSERT INTO ");
                self.from_item(&insert.table);
                if !insert.columns.is_empty() {
                    self.push(" (");
                    self.list(&insert.columns, ", ", |this, c| this.ident(c));
                    self.push(")");
                }
                if let Some(overriding) = insert.overriding {
                    self.push(" ");
                    self.push(overriding);
                }
                match &insert.source {
                    InsertSource::Values(rows) => {
                        self.push(" ");
                        self.stmt(&Stmt::Values(rows.clone()));
                    }
                    InsertSource::Query(query) => {
                        self.push(" ");
                        self.stmt(query);
                    }
                    InsertSource::DefaultValues => self.push(" DEFAULT VALUES"),
                }
                if let Some(conflict) = &insert.on_conflict {
                    self.on_conflict(conflict);
                }
                self.returning(&insert.returning);
            }
            Stmt::Update(update) => {
                self.push("UPDATE ");
                self.from_item(&update.table);
                self.assignments(&update.set);
                if !update.from.is_empty() {
                    self.push(" FROM ");
                    self.joins(&update.from);
                }
                self.filter(&update.filter);
                self.returning(&update.returning);
            }
            Stmt::Delete(delete) => {
                self.push("DELETE FROM ");
                self.from_item(&delete.table);
                if !delete.using.is_empty() {
                    self.push(" USING ");
                    self.joins(&delete.using);
                }
                self.filter(&delete.filter);
                self.returning(&delete.returning);
            }
            Stmt::Merge(merge) => {
                self.push("MERGE INTO ");
                self.from_item(&merge.table);
                self.push(" USING ");
                self.from_item(&merge.source);
                self.push(" ON ");
                self.node(&merge.on);
                for when in &merge.whens {
                    self.push(" ");
                    self.node(when);
                }
                self.returning(&merge.returning);
            }
        }
    }

    fn on_conflict(&mut self, conflict: &OnConflict) {
        self.push(" ON CONFLICT");
        if !conflict.target.is_empty() {
            self.push(" ");
            self.list(&conflict.target, " ", Self::node);
        }
        match &conflict.update {
            None => self.push(" DO NOTHING"),
            Some(set) => {
                self.push(" DO UPDATE");
                self.assignments(set);
                self.filter(&conflict.filter);
            }
        }
    }
}
