use core::fmt;
use std::{io::{stdin, Read}, collections::HashMap};

mod ast;

use ast::*;

#[derive(Debug, Clone)]
pub struct Error {
    pub start: usize,
    pub end: usize,
    pub message: String,
}

impl Error {
    pub fn new(message: &str, location: Location) -> Self {
        Self {
            start: location.start,
            end: location.end,
            message: String::from(message),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Closure {
    pub body: Term,
    pub args: Vec<Parameter>,
    pub scope: HashMap<String, Printable>,
}

#[derive(Debug, Clone)]
pub enum Printable {
    Void,
    Bool(bool),
    Int(i32),
    Str(String),
    Tuple((Box<Printable>, Box<Printable>)),
    Closure(Closure),
}

impl fmt::Display for Printable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Printable::Bool(x) => write!(f, "{}", x),
            Printable::Int(x) => write!(f, "{}", x),
            Printable::Str(x) => write!(f, "{}", x),
            Printable::Closure(x) => write!(f, "{:?}", x),
            _ => Ok(()),
        }
    }
}

fn eval(term: Term, scope: &mut HashMap<String, Printable>) -> Result<Printable, Error> {
    let res = match term {
        Term::Bool(x) => Printable::Bool(x.value),
        Term::Int(x) => Printable::Int(x.value),
        Term::Str(x) => Printable::Str(x.value),
        Term::Print(x) => {
            return eval(*x.value, scope);
        }
        Term::Binary(x) => {
            let lhs = eval(*x.lhs, scope).unwrap();
            let rhs = eval(*x.rhs, scope).unwrap();

            match x.op {
                BinaryOp::Add => match (lhs, rhs) {
                    (Printable::Int(a), Printable::Int(b)) => Printable::Int(a + b),
                    (Printable::Str(a), Printable::Str(b)) => Printable::Str(format!("{}{}", a, b)),
                    (Printable::Str(a), Printable::Int(b)) => Printable::Str(format!("{}{}", a, b)),
                    (Printable::Int(a), Printable::Str(b)) => Printable::Str(format!("{}{}", a, b)),
                    _ => return Err(Error::new("Cannot perform add operation", x.location)),
                },
                BinaryOp::Sub => match (lhs, rhs) {
                    (Printable::Int(a), Printable::Int(b)) => Printable::Int(a - b),
                    _ => return Err(Error::new("Cannot perform sub operation", x.location)),
                },
                BinaryOp::Mul => match (lhs, rhs) {
                    (Printable::Int(a), Printable::Int(b)) => Printable::Int(a * b),
                    _ => return Err(Error::new("Cannot perform mul operation", x.location)),
                },
                BinaryOp::Div => match (lhs, rhs) {
                    (Printable::Int(a), Printable::Int(b)) => {
                        if b > 0 {
                            Printable::Int(a / b)
                        } else {
                            return Err(Error::new("Arithmetic error dividing by zero", x.location));
                        }
                    }
                    _ => return Err(Error::new("Cannot perform div operation", x.location)),
                },
                _ => Printable::Void,
            }
        }
        Term::If(x) => {
            let cond = eval(*x.condition, scope).unwrap();

            match cond {
                Printable::Bool(true) => return eval(*x.then, scope),
                Printable::Bool(false) => return eval(*x.otherwise, scope),
                _ => {
                    return Err(Error::new(
                        "Condition expression not resolve to a boolean primitive",
                        x.location,
                    ))
                }
            }
        }
        Term::Tuple(x) => {
            let _1st = eval(*x.first, scope).unwrap();
            let _2nd = eval(*x.second, scope).unwrap();

            Printable::Tuple((Box::new(_1st), Box::new(_2nd)))
        },
        Term::First(x) => {
            let val = eval(*x.value, scope).unwrap();

            match val {
                Printable::Tuple(x) => {
                    let _1st = x.0;

                    panic!();
                },
                _ => panic!(),
            }
        },
        Term::Second(x) => {
            let val = eval(*x.value, scope).unwrap();
            panic!()
        },
        Term::Var(x) => {
            if let Some(var) = scope.get(&x.text) {
                var.clone()
            } else {
                let msg = format!("Variable {} is not declared in this scope", &x.text);

                return Err(Error::new(msg.as_str(), x.location));
            }
        },
        Term::Let(x) => {
            let id = x.name.text;
            let val = eval(*x.value, scope).unwrap();
            scope.insert(id, val);

            return eval(*x.next, scope);
        },
        Term::Call(x) => {
            let mut inner_scope = scope.clone();
            let func = eval(*x.callee, scope).unwrap();

            match func {
                Printable::Closure(y) => {
                    for (param, arg) in y.args.into_iter().zip(x.arguments) {
                        inner_scope.insert(param.text, eval(arg, scope).unwrap());
                    }

                    return eval(y.body, &mut inner_scope);
                },
                _ => return Err(Error::new("Calling a not callable", x.location)),
            }
        },
        Term::Function(x) => {
            Printable::Closure(Closure { body: *x.value, args: x.parameters, scope: scope.clone() })
        },
    };

    Ok(res)
}

fn main() {
    let mut prog = String::new();
    stdin().lock().read_to_string(&mut prog).unwrap();
    let prog = serde_json::from_str::<File>(&prog).unwrap();

    let expr = prog.expression;
    let mut scope = HashMap::new();

    match eval(expr, &mut scope) {
        Ok(printable) => match printable {
            Printable::Bool(x) => println!("{}", x),
            Printable::Int(x) => println!("{}", x),
            Printable::Str(x) => println!("{}", x),
            Printable::Tuple(x) => println!("({}, {})", x.0, x.1),
            Printable::Closure(x) => println!("<#{:?}>", x),
            Printable::Void => (),
        },
        // FONT: lineno === start and column === end, see: '[0]
        //
        // '[0]: https://www.gnu.org/prep/standards/standards.html#Errors
        Err(err) => println!(
            "RINHA_IMPL_ERROR:{}:{}: {}",
            err.start, err.end, err.message
        ),
    }
}
