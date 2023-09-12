use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Location {
    pub start: usize,
    pub end: usize,
    pub filename: String,
}

#[derive(Debug, Clone, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Binary {
    pub lhs: Box<Term>,
    pub op: BinaryOp,
    pub rhs: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Bool {
    pub value: bool,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Call {
    pub callee: Box<Term>,
    pub arguments: Vec<Term>,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct First {
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Var {
    pub text: String,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Parameter {
    pub text: String,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Function {
    pub parameters: Vec<Parameter>,
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct If {
    pub condition: Box<Term>,
    pub then: Box<Term>,
    pub otherwise: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Int {
    pub value: i32,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Let {
    pub name: Parameter,
    pub value: Box<Term>,
    pub next: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Print {
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Second {
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Str {
    pub value: String,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tuple {
    pub first: Box<Term>,
    pub second: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    Binary(Binary),
    Bool(Bool),
    Call(Call),
    First(First),
    Function(Function),
    If(If),
    Int(Int),
    Let(Let),
    Print(Print),
    Second(Second),
    Str(Str),
    Tuple(Tuple),
    Var(Var),
}

#[derive(Debug, Deserialize)]
pub struct File {
    pub name: String,
    pub expression: Term,
    pub location: Location,
}
