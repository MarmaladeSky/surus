//! The query tree built by the typed DSL and serialized by the renderer.
//!
//! Types are checked by the builders; this tree only keeps the structure of
//! the statement. Identifiers are names, never SQL text: the renderer quotes
//! them, and values are always rendered as parameters.

use crate::Arg;
use std::sync::Arc;

/// What a column reference is qualified by.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rel {
    /// A FROM item, CTE or DML target, identified by a unique id.
    Source(u64),
    Old,
    New,
    Excluded,
    /// Key `index` of the GROUP BY clause with the given id.
    Key(u64, usize),
    /// Output column `index` of a set operation, in its ORDER BY.
    Output(usize),
}

#[derive(Clone, Debug)]
pub enum Node {
    Column(Rel, &'static str),
    Star(Rel),
    Param(usize),
    Value(Arg),
    /// A string constant, where the grammar admits no parameter.
    Literal(&'static str),
    Null,
    Default,
    Keyword(&'static str),
    Type(String),
    Binary(&'static str, Box<Node>, Box<Node>),
    Prefix(&'static str, Box<Node>),
    Postfix(Box<Node>, &'static str),
    Call(&'static str, Vec<Node>),
    /// A named function argument, `name => value`.
    Named(&'static str, Box<Node>),
    Cast(Box<Node>, String),
    Case(Option<Box<Node>>, Vec<(Node, Node)>, Option<Box<Node>>),
    Array(Vec<Node>),
    Row(Vec<Node>),
    /// A parenthesized list, as in `IN (a, b)`.
    List(Vec<Node>),
    Subquery(Box<Statement>),
    /// `ANY` or `ALL` of an array or subquery.
    Quantified(&'static str, Box<Node>),
    Subscript(Box<Node>, Box<Node>, Option<Box<Node>>),
    Field(Box<Node>, &'static str),
    FieldStar(Box<Node>),
    Collate(Box<Node>, &'static str),
    Aggregate(Box<Aggregate>),
    Over(Box<Node>, Box<Window>),
    /// Special syntax: a function whose arguments are separated by keywords,
    /// such as `extract(year FROM x)` or `JSON_VALUE(x, p RETURNING int4)`.
    Syntax(&'static str, Vec<Node>),
    /// Keyword-separated parts without parentheses, as in `x IS JSON OBJECT`.
    Seq(Vec<Node>),
}

impl Node {
    pub fn binary(op: &'static str, left: Node, right: Node) -> Node {
        Node::Binary(op, Box::new(left), Box::new(right))
    }
}

#[derive(Clone, Debug)]
pub struct Aggregate {
    pub name: &'static str,
    pub args: Vec<Node>,
    pub distinct: bool,
    pub order: Vec<Order>,
    pub within_group: Vec<Order>,
    pub filter: Option<Node>,
}

#[derive(Clone, Debug, Default)]
pub struct Window {
    pub partition: Vec<Node>,
    pub order: Vec<Order>,
    pub frame: Option<Frame>,
}

#[derive(Clone, Debug)]
pub struct Frame {
    pub units: &'static str,
    pub start: Bound,
    pub end: Bound,
    pub exclude: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub enum Bound {
    UnboundedPreceding,
    Preceding(Node),
    CurrentRow,
    Following(Node),
    UnboundedFollowing,
}

#[derive(Clone, Debug)]
pub struct Order {
    pub node: Node,
    pub direction: Option<&'static str>,
    pub nulls: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Select(Box<Select>),
    Compound(Box<Compound>),
    Values(Vec<Vec<Node>>),
    Insert(Box<Insert>),
    Update(Box<Update>),
    Delete(Box<Delete>),
    Merge(Box<Merge>),
}

#[derive(Clone, Debug, Default)]
pub struct Select {
    /// `Some(vec![])` is DISTINCT, a non-empty list DISTINCT ON.
    pub distinct: Option<Vec<Node>>,
    pub items: Vec<Item>,
    pub from: Vec<Join>,
    pub filter: Option<Node>,
    pub group: Option<Group>,
    pub having: Option<Node>,
    pub order: Vec<Order>,
    pub limit: Option<Node>,
    pub offset: Option<Node>,
    pub fetch: Option<(Node, bool)>,
    pub locks: Vec<Lock>,
}

#[derive(Clone, Debug)]
pub struct Item {
    pub node: Node,
    pub alias: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub struct Join {
    /// Empty for the first FROM item.
    pub kind: &'static str,
    pub lateral: bool,
    pub from: From,
    pub on: Option<Node>,
}

#[derive(Clone, Debug)]
pub struct From {
    pub source: Source,
    pub rel: u64,
    /// Number of positional output columns, named in a column alias list.
    pub columns: usize,
}

#[derive(Clone, Debug)]
pub enum Source {
    Table {
        schema: &'static str,
        name: &'static str,
        sample: Option<Sample>,
    },
    Query(Statement),
    Cte(Arc<Cte>),
    /// A recursive CTE referenced from its own definition.
    Recursive(u64),
    Function {
        calls: Vec<Node>,
        rows_from: bool,
        ordinality: bool,
    },
}

#[derive(Clone, Debug)]
pub struct Sample {
    pub method: &'static str,
    pub args: Vec<Node>,
    pub repeatable: Option<Node>,
}

#[derive(Debug)]
pub struct Cte {
    pub id: u64,
    pub body: Statement,
    pub columns: usize,
    pub recursive: bool,
    pub materialized: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct Group {
    pub id: u64,
    /// Empty for a plain list, else ROLLUP, CUBE or GROUPING SETS.
    pub kind: &'static str,
    pub keys: Vec<Node>,
    /// Key indices of each grouping set, for GROUPING SETS.
    pub sets: Vec<Vec<usize>>,
}

#[derive(Clone, Debug)]
pub struct Lock {
    pub strength: &'static str,
    pub of: Vec<u64>,
    pub wait: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub struct Compound {
    pub op: &'static str,
    pub left: Statement,
    pub right: Statement,
    pub order: Vec<Order>,
    pub limit: Option<Node>,
    pub offset: Option<Node>,
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub columns: Vec<&'static str>,
    pub value: Node,
}

#[derive(Clone, Debug)]
pub enum Conflict {
    Columns(Vec<Node>, Option<Node>),
    Constraint(&'static str),
}

#[derive(Clone, Debug)]
pub struct OnConflict {
    pub target: Option<Conflict>,
    /// `None` is DO NOTHING.
    pub update: Option<(Vec<Assignment>, Option<Node>)>,
}

#[derive(Clone, Debug)]
pub struct Insert {
    pub target: From,
    pub columns: Vec<&'static str>,
    pub overriding: bool,
    /// VALUES rows, or `None` with a query or for DEFAULT VALUES.
    pub rows: Vec<Vec<Node>>,
    pub query: Option<Statement>,
    pub conflict: Option<OnConflict>,
    pub returning: Vec<Item>,
}

#[derive(Clone, Debug)]
pub struct Update {
    pub target: From,
    pub set: Vec<Assignment>,
    pub from: Vec<Join>,
    pub filter: Option<Node>,
    pub returning: Vec<Item>,
}

#[derive(Clone, Debug)]
pub struct Delete {
    pub target: From,
    pub using: Vec<Join>,
    pub filter: Option<Node>,
    pub returning: Vec<Item>,
}

#[derive(Clone, Debug)]
pub struct Merge {
    pub target: From,
    pub source: From,
    pub on: Node,
    pub clauses: Vec<When>,
    pub returning: Vec<Item>,
}

#[derive(Clone, Debug)]
pub struct When {
    pub kind: &'static str,
    pub condition: Option<Node>,
    pub action: Action,
}

#[derive(Clone, Debug)]
pub enum Action {
    Update(Vec<Assignment>),
    Insert(Vec<&'static str>, Vec<Node>),
    Delete,
    Nothing,
}
