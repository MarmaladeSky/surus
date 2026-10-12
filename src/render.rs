//! Serializes the query tree into PostgreSQL SQL with numbered parameters.

use crate::Arg;
use crate::ast::*;
use std::fmt::Write;

pub(crate) fn render(statement: &Statement, params: usize) -> (String, Vec<Arg>) {
    let mut r = Renderer {
        sql: String::with_capacity(256),
        args: Vec::new(),
        params,
        names: Vec::new(),
        groups: Vec::new(),
        ctes: Vec::new(),
        recursive: false,
        bare: false,
    };
    r.statement(statement);
    if r.ctes.is_empty() {
        return (r.sql, r.args);
    }
    let mut sql = String::from(if r.recursive {
        "WITH RECURSIVE "
    } else {
        "WITH "
    });
    for (i, (_, cte)) in r.ctes.iter().enumerate() {
        if i > 0 {
            sql.push_str(", ");
        }
        sql.push_str(cte);
    }
    sql.push(' ');
    sql.push_str(&r.sql);
    (sql, r.args)
}

struct Renderer<'a> {
    sql: String,
    args: Vec<Arg>,
    params: usize,
    /// Source ids in order of first use; the alias of an id is its position.
    names: Vec<u64>,
    groups: Vec<&'a Group>,
    /// Rendered CTE definitions, dependencies first.
    ctes: Vec<(u64, String)>,
    recursive: bool,
    /// Render columns unqualified, as in SET and ON CONFLICT targets.
    bare: bool,
}

fn quote(out: &mut String, name: &str) {
    out.push('"');
    for c in name.chars() {
        if c == '"' {
            out.push('"');
        }
        out.push(c);
    }
    out.push('"');
}

impl<'a> Renderer<'a> {
    fn push(&mut self, s: &str) {
        self.sql.push_str(s);
    }

    fn name(&mut self, id: u64) {
        let index = match self.names.iter().position(|n| *n == id) {
            Some(index) => index,
            None => {
                self.names.push(id);
                self.names.len() - 1
            }
        };
        write!(self.sql, "t{}", index + 1).unwrap();
    }

    fn quoted(&mut self, name: &str) {
        quote(&mut self.sql, name);
    }

    fn list<T>(&mut self, items: &'a [T], mut f: impl FnMut(&mut Self, &'a T)) {
        for (i, item) in items.iter().enumerate() {
            if i > 0 {
                self.push(", ");
            }
            f(self, item);
        }
    }

    fn nodes(&mut self, nodes: &'a [Node]) {
        self.list(nodes, Self::node);
    }

    fn rel(&mut self, rel: Rel) {
        match rel {
            Rel::Source(id) => self.name(id),
            Rel::Old => self.push("old"),
            Rel::New => self.push("new"),
            Rel::Excluded => self.push("excluded"),
            Rel::Key(..) | Rel::Output(_) => unreachable!("not a relation"),
        }
    }

    fn node(&mut self, node: &'a Node) {
        match node {
            Node::Column(Rel::Key(id, index), _) => {
                let group = self.groups.iter().rev().find(|g| g.id == *id);
                let key = &group.expect("grouping key outside its query").keys[*index];
                self.node(key);
            }
            Node::Column(Rel::Output(index), _) => {
                write!(self.sql, "{}", index + 1).unwrap();
            }
            Node::Column(rel, name) => {
                if !self.bare {
                    self.rel(*rel);
                    self.push(".");
                }
                self.quoted(name);
            }
            Node::Star(rel) => {
                self.rel(*rel);
                self.push(".*");
            }
            Node::Param(index) => write!(self.sql, "${}", index + 1).unwrap(),
            Node::Value(arg) => {
                self.args.push(arg.clone());
                write!(self.sql, "${}", self.params + self.args.len()).unwrap();
            }
            Node::Literal(text) => {
                self.push("'");
                self.push(&text.replace('\'', "''"));
                self.push("'");
            }
            Node::Null => self.push("NULL"),
            Node::Default => self.push("DEFAULT"),
            Node::Keyword(keyword) => self.push(keyword),
            Node::Type(name) => self.push(name),
            Node::Binary(op, left, right) => {
                self.push("(");
                self.node(left);
                self.push(" ");
                self.push(op);
                self.push(" ");
                self.node(right);
                self.push(")");
            }
            Node::Prefix(op, operand) => {
                self.push("(");
                self.push(op);
                self.push(" ");
                self.node(operand);
                self.push(")");
            }
            Node::Postfix(operand, op) => {
                self.push("(");
                self.node(operand);
                self.push(" ");
                self.push(op);
                self.push(")");
            }
            Node::Call(name, args) => {
                self.push(name);
                self.push("(");
                self.nodes(args);
                self.push(")");
            }
            Node::Named(name, value) => {
                self.push(name);
                self.push(" => ");
                self.node(value);
            }
            Node::Cast(operand, ty) => {
                self.push("CAST(");
                self.node(operand);
                self.push(" AS ");
                self.push(ty);
                self.push(")");
            }
            Node::Case(operand, whens, otherwise) => {
                self.push("CASE");
                if let Some(operand) = operand {
                    self.push(" ");
                    self.node(operand);
                }
                for (when, then) in whens {
                    self.push(" WHEN ");
                    self.node(when);
                    self.push(" THEN ");
                    self.node(then);
                }
                if let Some(otherwise) = otherwise {
                    self.push(" ELSE ");
                    self.node(otherwise);
                }
                self.push(" END");
            }
            Node::Array(items) => {
                self.push("ARRAY[");
                self.nodes(items);
                self.push("]");
            }
            Node::Row(items) => {
                self.push("ROW(");
                self.nodes(items);
                self.push(")");
            }
            Node::List(items) => {
                self.push("(");
                self.nodes(items);
                self.push(")");
            }
            Node::Subquery(statement) => {
                self.push("(");
                self.statement(statement);
                self.push(")");
            }
            Node::Quantified(quantifier, operand) => {
                self.push(quantifier);
                if let Node::Subquery(_) = **operand {
                    self.push(" ");
                    self.node(operand);
                } else {
                    self.push("(");
                    self.node(operand);
                    self.push(")");
                }
            }
            Node::Subscript(array, index, upper) => {
                self.push("(");
                self.node(array);
                self.push(")[");
                self.node(index);
                if let Some(upper) = upper {
                    self.push(":");
                    self.node(upper);
                }
                self.push("]");
            }
            Node::Field(operand, field) => {
                self.push("(");
                self.node(operand);
                self.push(").");
                self.quoted(field);
            }
            Node::FieldStar(operand) => {
                self.push("(");
                self.node(operand);
                self.push(").*");
            }
            Node::Collate(operand, collation) => {
                self.push("(");
                self.node(operand);
                self.push(" COLLATE ");
                self.quoted(collation);
                self.push(")");
            }
            Node::Aggregate(aggregate) => self.aggregate(aggregate),
            Node::Over(call, window) => {
                self.node(call);
                self.push(" OVER (");
                self.window(window);
                self.push(")");
            }
            Node::Syntax(name, parts) => {
                self.push(name);
                self.push("(");
                self.seq(parts);
                self.push(")");
            }
            Node::Seq(parts) => {
                self.push("(");
                self.seq(parts);
                self.push(")");
            }
        }
    }

    fn seq(&mut self, parts: &'a [Node]) {
        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                self.push(" ");
            }
            self.node(part);
        }
    }

    fn aggregate(&mut self, a: &'a Aggregate) {
        self.push(a.name);
        self.push("(");
        if a.distinct {
            self.push("DISTINCT ");
        }
        if a.args.is_empty() && a.within_group.is_empty() && a.name == "count" {
            self.push("*");
        }
        self.nodes(&a.args);
        if !a.order.is_empty() {
            self.push(" ORDER BY ");
            self.orders(&a.order);
        }
        self.push(")");
        if !a.within_group.is_empty() {
            self.push(" WITHIN GROUP (ORDER BY ");
            self.orders(&a.within_group);
            self.push(")");
        }
        if let Some(filter) = &a.filter {
            self.push(" FILTER (WHERE ");
            self.node(filter);
            self.push(")");
        }
    }

    fn window(&mut self, w: &'a Window) {
        let mut space = "";
        if !w.partition.is_empty() {
            self.push("PARTITION BY ");
            self.nodes(&w.partition);
            space = " ";
        }
        if !w.order.is_empty() {
            self.push(space);
            self.push("ORDER BY ");
            self.orders(&w.order);
            space = " ";
        }
        if let Some(frame) = &w.frame {
            self.push(space);
            self.push(frame.units);
            self.push(" BETWEEN ");
            self.bound(&frame.start);
            self.push(" AND ");
            self.bound(&frame.end);
            if let Some(exclude) = frame.exclude {
                self.push(" EXCLUDE ");
                self.push(exclude);
            }
        }
    }

    fn bound(&mut self, bound: &'a Bound) {
        match bound {
            Bound::UnboundedPreceding => self.push("UNBOUNDED PRECEDING"),
            Bound::Preceding(offset) => {
                self.node(offset);
                self.push(" PRECEDING");
            }
            Bound::CurrentRow => self.push("CURRENT ROW"),
            Bound::Following(offset) => {
                self.node(offset);
                self.push(" FOLLOWING");
            }
            Bound::UnboundedFollowing => self.push("UNBOUNDED FOLLOWING"),
        }
    }

    fn orders(&mut self, orders: &'a [Order]) {
        self.list(orders, |r, order| {
            r.node(&order.node);
            if let Some(direction) = order.direction {
                r.push(" ");
                r.push(direction);
            }
            if let Some(nulls) = order.nulls {
                r.push(" NULLS ");
                r.push(nulls);
            }
        });
    }

    fn items(&mut self, items: &'a [Item]) {
        self.list(items, |r, item| {
            r.node(&item.node);
            if let Some(alias) = item.alias {
                r.push(" AS ");
                r.quoted(alias);
            }
        });
    }

    fn statement(&mut self, statement: &'a Statement) {
        match statement {
            Statement::Select(select) => self.select(select),
            Statement::Compound(compound) => self.compound(compound),
            Statement::Values(rows) => self.values(rows),
            Statement::Insert(insert) => self.insert(insert),
            Statement::Update(update) => self.update(update),
            Statement::Delete(delete) => self.delete(delete),
            Statement::Merge(merge) => self.merge(merge),
        }
    }

    fn values(&mut self, rows: &'a [Vec<Node>]) {
        self.push("VALUES ");
        self.list(rows, |r, row| {
            r.push("(");
            r.nodes(row);
            r.push(")");
        });
    }

    fn select(&mut self, s: &'a Select) {
        if let Some(group) = &s.group {
            self.groups.push(group);
        }
        self.push("SELECT ");
        match &s.distinct {
            Some(on) if on.is_empty() => self.push("DISTINCT "),
            Some(on) => {
                self.push("DISTINCT ON (");
                self.nodes(on);
                self.push(") ");
            }
            None => {}
        }
        self.items(&s.items);
        if !s.from.is_empty() {
            self.push(" FROM ");
            self.joins(&s.from);
        }
        self.filter(" WHERE ", s.filter.as_ref());
        if let Some(group) = &s.group {
            self.push(" GROUP BY ");
            self.group(group);
        }
        self.filter(" HAVING ", s.having.as_ref());
        self.tail(&s.order, s.limit.as_ref(), s.offset.as_ref());
        if let Some((count, ties)) = &s.fetch {
            self.push(" FETCH FIRST ");
            self.node(count);
            self.push(if *ties {
                " ROWS WITH TIES"
            } else {
                " ROWS ONLY"
            });
        }
        for lock in &s.locks {
            self.push(" FOR ");
            self.push(lock.strength);
            if !lock.of.is_empty() {
                self.push(" OF ");
                self.list(&lock.of, |r, id| r.name(*id));
            }
            if let Some(wait) = lock.wait {
                self.push(" ");
                self.push(wait);
            }
        }
        if s.group.is_some() {
            self.groups.pop();
        }
    }

    fn filter(&mut self, keyword: &str, filter: Option<&'a Node>) {
        if let Some(filter) = filter {
            self.push(keyword);
            self.node(filter);
        }
    }

    fn tail(&mut self, order: &'a [Order], limit: Option<&'a Node>, offset: Option<&'a Node>) {
        if !order.is_empty() {
            self.push(" ORDER BY ");
            self.orders(order);
        }
        self.filter(" LIMIT ", limit);
        if let Some(offset) = offset {
            self.push(" OFFSET ");
            self.node(offset);
            self.push(" ROWS");
        }
    }

    fn group(&mut self, group: &'a Group) {
        match group.kind {
            "" => self.nodes(&group.keys),
            "GROUPING SETS" => {
                self.push("GROUPING SETS (");
                self.list(&group.sets, |r, set| {
                    r.push("(");
                    r.list(set, |r, key| r.node(&group.keys[*key]));
                    r.push(")");
                });
                self.push(")");
            }
            kind => {
                self.push(kind);
                self.push(" (");
                self.list(&group.keys, |r, key| {
                    r.push("(");
                    r.node(key);
                    r.push(")");
                });
                self.push(")");
            }
        }
    }

    fn joins(&mut self, joins: &'a [Join]) {
        for join in joins {
            if !join.kind.is_empty() {
                self.push(" ");
                self.push(join.kind);
                self.push(" ");
            }
            if join.lateral {
                self.push("LATERAL ");
            }
            self.from(&join.from);
            self.filter(" ON ", join.on.as_ref());
        }
    }

    fn from(&mut self, from: &'a From) {
        match &from.source {
            Source::Table {
                schema,
                name,
                sample,
            } => {
                self.quoted(schema);
                self.push(".");
                self.quoted(name);
                self.push(" AS ");
                self.name(from.rel);
                if let Some(sample) = sample {
                    self.push(" TABLESAMPLE ");
                    self.push(sample.method);
                    self.push(" (");
                    self.nodes(&sample.args);
                    self.push(")");
                    if let Some(seed) = &sample.repeatable {
                        self.push(" REPEATABLE (");
                        self.node(seed);
                        self.push(")");
                    }
                }
                return;
            }
            Source::Query(statement) => {
                self.push("(");
                self.statement(statement);
                self.push(")");
            }
            Source::Cte(cte) => {
                self.cte(cte);
                self.name(cte.id);
            }
            Source::Recursive(id) => self.name(*id),
            Source::Function {
                calls,
                rows_from,
                ordinality,
            } => {
                if *rows_from {
                    self.push("ROWS FROM (");
                    self.nodes(calls);
                    self.push(")");
                } else {
                    self.nodes(calls);
                }
                if *ordinality {
                    self.push(" WITH ORDINALITY");
                }
            }
        }
        self.push(" AS ");
        self.name(from.rel);
        self.columns(from.columns);
    }

    fn columns(&mut self, count: usize) {
        if count > 0 {
            self.push("(");
            for i in 1..=count {
                if i > 1 {
                    self.push(", ");
                }
                write!(self.sql, "c{i}").unwrap();
            }
            self.push(")");
        }
    }

    fn cte(&mut self, cte: &'a Cte) {
        if self.ctes.iter().any(|(id, _)| *id == cte.id) {
            return;
        }
        let outer = std::mem::take(&mut self.sql);
        self.ctes.push((cte.id, String::new()));
        let slot = self.ctes.len() - 1;
        self.name(cte.id);
        self.columns(cte.columns);
        self.push(match cte.materialized {
            Some(true) => " AS MATERIALIZED (",
            Some(false) => " AS NOT MATERIALIZED (",
            None => " AS (",
        });
        self.statement(&cte.body);
        self.push(")");
        self.recursive |= cte.recursive;
        let definition = std::mem::replace(&mut self.sql, outer);
        // dependencies rendered meanwhile come first
        self.ctes.remove(slot);
        self.ctes.push((cte.id, definition));
    }

    fn compound(&mut self, c: &'a Compound) {
        self.push("(");
        self.statement(&c.left);
        self.push(") ");
        self.push(c.op);
        self.push(" (");
        self.statement(&c.right);
        self.push(")");
        self.tail(&c.order, c.limit.as_ref(), c.offset.as_ref());
    }

    fn target(&mut self, target: &'a From) {
        if let Source::Table { schema, name, .. } = &target.source {
            self.quoted(schema);
            self.push(".");
            self.quoted(name);
            self.push(" AS ");
            self.name(target.rel);
        }
    }

    fn assignments(&mut self, set: &'a [Assignment]) {
        self.bare = true;
        self.list(set, |r, a| {
            if let [column] = a.columns[..] {
                r.quoted(column);
            } else {
                r.push("(");
                r.list(&a.columns, |r, c| r.quoted(c));
                r.push(")");
            }
            r.push(" = ");
            r.bare = false;
            r.node(&a.value);
            r.bare = true;
        });
        self.bare = false;
    }

    fn returning(&mut self, items: &'a [Item]) {
        if !items.is_empty() {
            self.push(" RETURNING ");
            self.items(items);
        }
    }

    fn insert(&mut self, i: &'a Insert) {
        self.push("INSERT INTO ");
        self.target(&i.target);
        if !i.columns.is_empty() {
            self.push(" (");
            self.list(&i.columns, |r, c| r.quoted(c));
            self.push(")");
        }
        if i.overriding {
            self.push(" OVERRIDING SYSTEM VALUE");
        }
        if let Some(query) = &i.query {
            self.push(" ");
            self.statement(query);
        } else if i.rows.is_empty() {
            self.push(" DEFAULT VALUES");
        } else {
            self.push(" ");
            self.values(&i.rows);
        }
        if let Some(conflict) = &i.conflict {
            self.push(" ON CONFLICT");
            match &conflict.target {
                Some(Conflict::Columns(columns, filter)) => {
                    self.bare = true;
                    self.push(" (");
                    self.nodes(columns);
                    self.push(")");
                    self.filter(" WHERE ", filter.as_ref());
                    self.bare = false;
                }
                Some(Conflict::Constraint(name)) => {
                    self.push(" ON CONSTRAINT ");
                    self.quoted(name);
                }
                None => {}
            }
            match &conflict.update {
                None => self.push(" DO NOTHING"),
                Some((set, filter)) => {
                    self.push(" DO UPDATE SET ");
                    self.assignments(set);
                    self.filter(" WHERE ", filter.as_ref());
                }
            }
        }
        self.returning(&i.returning);
    }

    fn sources(&mut self, keyword: &str, joins: &'a [Join]) {
        if !joins.is_empty() {
            self.push(keyword);
            self.list(joins, |r, join| {
                if join.lateral {
                    r.push("LATERAL ");
                }
                r.from(&join.from);
            });
        }
    }

    fn update(&mut self, u: &'a Update) {
        self.push("UPDATE ");
        self.target(&u.target);
        self.push(" SET ");
        self.assignments(&u.set);
        self.sources(" FROM ", &u.from);
        self.filter(" WHERE ", u.filter.as_ref());
        self.returning(&u.returning);
    }

    fn delete(&mut self, d: &'a Delete) {
        self.push("DELETE FROM ");
        self.target(&d.target);
        self.sources(" USING ", &d.using);
        self.filter(" WHERE ", d.filter.as_ref());
        self.returning(&d.returning);
    }

    fn merge(&mut self, m: &'a Merge) {
        self.push("MERGE INTO ");
        self.target(&m.target);
        self.push(" USING ");
        self.from(&m.source);
        self.push(" ON ");
        self.node(&m.on);
        for when in &m.clauses {
            self.push(" WHEN ");
            self.push(when.kind);
            self.filter(" AND ", when.condition.as_ref());
            self.push(" THEN ");
            match &when.action {
                Action::Update(set) => {
                    self.push("UPDATE SET ");
                    self.assignments(set);
                }
                Action::Insert(columns, values) => {
                    self.push("INSERT (");
                    self.list(columns, |r, c| r.quoted(c));
                    self.push(") VALUES (");
                    self.nodes(values);
                    self.push(")");
                }
                Action::Delete => self.push("DELETE"),
                Action::Nothing => self.push("DO NOTHING"),
            }
        }
        self.returning(&m.returning);
    }
}
