use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Location {
    pub start: usize,
    pub end: usize,
    pub filename: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
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

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Binary {
    pub lhs: Box<Term>,
    pub op: BinaryOp,
    pub rhs: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Bool {
    pub value: bool,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Call {
    pub callee: Box<Term>,
    pub arguments: Vec<Term>,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct First {
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Var {
    pub text: String,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Parameter {
    pub text: String,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Function {
    pub parameters: Vec<Parameter>,
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct If {
    pub condition: Box<Term>,
    pub then: Box<Term>,
    pub otherwise: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Int {
    pub value: i32,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Let {
    pub name: Parameter,
    pub value: Box<Term>,
    pub next: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Print {
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Second {
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Str {
    pub value: String,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Tuple {
    pub first: Box<Term>,
    pub second: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
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
