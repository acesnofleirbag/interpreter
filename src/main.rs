use core::fmt;
use std::{collections::HashMap, fs, path::Path};

mod ast;
mod fib;

use ast::*;
use fib::*;

#[derive(Debug, Clone)]
pub struct Error {
    pub start: usize,
    pub end: usize,
    pub filename: String,
    pub message: String,
}

impl Error {
    pub fn new(message: &str, location: Location) -> Self {
        Self {
            start: location.start,
            end: location.end,
            filename: location.filename,
            message: String::from(message),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    outter: Option<HashMap<String, Output>>,
    inner: HashMap<String, Output>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Closure {
    pub body: Term,
    pub args: Vec<Parameter>,
    pub context: Context,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    Bool(bool),
    Int(u128),
    Str(String),
    Tuple((Box<Output>, Box<Output>)),
    Closure(Closure),
    Void,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Output::Bool(x) => write!(f, "{}", x),
            Output::Int(x) => write!(f, "{}", x),
            Output::Str(x) => write!(f, "{}", x),
            _ => Ok(()),
        }
    }
}

fn eval(term: Term, context: &mut Context) -> Result<Output, Error> {
    match term {
        Term::Bool(x) => Ok(Output::Bool(x.value)),
        Term::Int(x) => Ok(Output::Int(x.value as u128)),
        Term::Str(x) => Ok(Output::Str(x.value)),
        Term::Print(x) => {
            let expr = eval(*x.value, context)?;

            match expr {
                Output::Bool(x) => println!("{}", x),
                Output::Int(x) => println!("{}", x),
                Output::Str(x) => println!("{}", x),
                Output::Tuple(x) => println!("({}, {})", x.0, x.1),
                Output::Closure(_) => println!("<#closure>"),
                _ => return Err(Error::new("Unsupported expression", x.location)),
            };

            Ok(Output::Void)
        }
        Term::Binary(x) => {
            // @@@: spawn thread
            let lhs = eval(*x.lhs, context)?;
            let rhs = eval(*x.rhs, context)?;

            match x.op {
                BinaryOp::Add => match (lhs, rhs) {
                    (Output::Int(a), Output::Int(b)) => Ok(Output::Int(a + b)),
                    (Output::Str(a), Output::Str(b)) => Ok(Output::Str(format!("{}{}", a, b))),
                    (Output::Str(a), Output::Int(b)) => Ok(Output::Str(format!("{}{}", a, b))),
                    (Output::Int(a), Output::Str(b)) => Ok(Output::Str(format!("{}{}", a, b))),
                    _ => Err(Error::new("Cannot perform add operation", x.location)),
                },
                BinaryOp::Sub => match (lhs, rhs) {
                    (Output::Int(a), Output::Int(b)) => Ok(Output::Int(a - b)),
                    _ => Err(Error::new("Cannot perform sub operation", x.location)),
                },
                BinaryOp::Mul => match (lhs, rhs) {
                    (Output::Int(a), Output::Int(b)) => Ok(Output::Int(a * b)),
                    _ => Err(Error::new("Cannot perform mul operation", x.location)),
                },
                BinaryOp::Div => match (lhs, rhs) {
                    (Output::Int(a), Output::Int(b)) => {
                        if b > 0 {
                            Ok(Output::Int(a / b))
                        } else {
                            Err(Error::new("Arithmetic error, dividing by zero", x.location))
                        }
                    }
                    _ => Err(Error::new("Cannot perform div operation", x.location)),
                },
                BinaryOp::Eq => match (lhs, rhs) {
                    (a, b) => Ok(Output::Bool(a == b)),
                },
                BinaryOp::Neq => match (lhs, rhs) {
                    (a, b) => Ok(Output::Bool(a != b)),
                },
                BinaryOp::Gt => match (lhs, rhs) {
                    (Output::Int(a), Output::Int(b)) => Ok(Output::Bool(a > b)),
                    (Output::Str(a), Output::Str(b)) => Ok(Output::Bool(a > b)),
                    _ => Err(Error::new("Cannot perform gt operation", x.location)),
                },
                BinaryOp::Lt => match (lhs, rhs) {
                    (Output::Int(a), Output::Int(b)) => Ok(Output::Bool(a < b)),
                    (Output::Str(a), Output::Str(b)) => Ok(Output::Bool(a < b)),
                    _ => Err(Error::new("Cannot perform lt operation", x.location)),
                },
                BinaryOp::Gte => match (lhs, rhs) {
                    (Output::Int(a), Output::Int(b)) => Ok(Output::Bool(a >= b)),
                    (Output::Str(a), Output::Str(b)) => Ok(Output::Bool(a >= b)),
                    _ => Err(Error::new("Cannot perform gte operation", x.location)),
                },
                BinaryOp::Lte => match (lhs, rhs) {
                    (Output::Int(a), Output::Int(b)) => Ok(Output::Bool(a <= b)),
                    (Output::Str(a), Output::Str(b)) => Ok(Output::Bool(a <= b)),
                    _ => Err(Error::new("Cannot perform lte operation", x.location)),
                },
                BinaryOp::Rem => match (lhs, rhs) {
                    (Output::Int(a), Output::Int(b)) => {
                        if b > 0 {
                            Ok(Output::Int(a % b))
                        } else {
                            Err(Error::new("Arithmetic error, dividing by zero", x.location))
                        }
                    }
                    _ => Err(Error::new("Cannot perform rem operation", x.location)),
                },
                BinaryOp::And => match (lhs, rhs) {
                    (Output::Bool(false), _) => Ok(Output::Bool(false)),
                    (_, b) => Ok(b),
                },
                BinaryOp::Or => match (lhs, rhs) {
                    (Output::Bool(true), _) => Ok(Output::Bool(true)),
                    (_, b) => Ok(b),
                },
            }
        }
        Term::If(x) => {
            let cond = eval(*x.condition, context)?;

            match cond {
                Output::Bool(true) => eval(*x.then, context),
                Output::Bool(false) => eval(*x.otherwise, context),
                _ => Err(Error::new(
                    "Condition expression not resolve to a boolean primitive",
                    x.location,
                )),
            }
        }
        Term::Tuple(x) => {
            // @@@: spawn thread
            let _1st = eval(*x.first, context)?;
            let _2nd = eval(*x.second, context)?;

            Ok(Output::Tuple((Box::new(_1st), Box::new(_2nd))))
        }
        Term::First(x) => {
            let val = eval(*x.value, context)?;

            if let Output::Tuple(x) = val {
                Ok(*x.0)
            } else {
                Err(Error::new(
                    "Cannot access first of a non tuple argument",
                    x.location,
                ))
            }
        }
        Term::Second(x) => {
            let val = eval(*x.value, context)?;

            if let Output::Tuple(x) = val {
                Ok(*x.1)
            } else {
                Err(Error::new(
                    "Cannot access second of a non tuple argument",
                    x.location,
                ))
            }
        }
        Term::Var(x) => {
            if let Some(var) = context.inner.get(&x.text) {
                Ok(var.clone())
            } else if let Some(outter) = &context.outter {
                if let Some(var) = outter.get(&x.text) {
                    Ok(var.clone())
                } else {
                    let msg = format!("Variable {} is not declared", &x.text);

                    Err(Error::new(msg.as_str(), x.location))
                }
            } else {
                let msg = format!("Variable {} is not declared", &x.text);

                Err(Error::new(msg.as_str(), x.location))
            }
        }
        Term::Let(x) => {
            let id = x.name.text;
            let expr = eval(*x.value, context)?;

            if let Some(_) = &mut context.outter {
                () // Outter context already declared
            } else {
                context.outter = Some(HashMap::new());
            }

            // FIXME: context.inner
            match expr {
                Output::Closure(y) => {
                    let closure = Output::Closure(Closure {
                        body: y.body,
                        args: y.args,
                        context: context.clone(),
                    });

                    context.outter.as_mut().unwrap().insert(id, closure);
                }
                y => {
                    context.outter.as_mut().unwrap().insert(id, y);
                }
            }

            eval(*x.next, context)
        }
        Term::Call(x) => {
            let mut new_context = Context {
                outter: context.outter.clone(),
                inner: HashMap::new(),
            };

            if let Term::Var(z) = *x.callee.clone() {
                if z.text == "fib" {
                    // FIXME: integer overflow ???
                    if let Output::Int(nth) = eval(x.arguments[0].clone(), context)? {
                        let res: u128;

                        // @@@: test others algorithms
                        if nth < 1000 {
                            res = __fib_iter(nth);
                        } else {
                            res = __fib_matrix(nth);
                        }

                        return Ok(Output::Int(res));
                    }
                }
            }

            let func = eval(*x.callee, context)?;

            match func {
                Output::Closure(y) => {
                    if y.args.len() != x.arguments.len() {
                        return Err(Error::new(
                            "Arguments declaration differs parameters declaration",
                            x.location,
                        ));
                    }

                    for (param, arg) in y.args.into_iter().zip(x.arguments.clone()) {
                        new_context.inner.insert(param.text, eval(arg, context)?);
                    }

                    eval(y.body, &mut new_context)
                }
                _ => Err(Error::new("Calling a not callable", x.location)),
            }
        }
        Term::Function(x) => Ok(Output::Closure(Closure {
            body: *x.value,
            args: x.parameters,
            context: context.clone(),
        })),
    }
}

fn read_json(path: &str) -> File {
    let prog = fs::read_to_string(Path::new(path)).expect("Cannot read the program file");

    serde_json::from_str::<File>(&prog).unwrap()
}

fn main() {
    let prog = read_json("/var/rinha/source.rinha.json");
    let expr = prog.expression;

    let mut context = Context {
        outter: None,
        inner: HashMap::new(),
    };

    eval(expr, &mut context).unwrap_or_else(|err| {
        // FONT: lineno == start and column == end, see: '[0]
        //
        // '[0]: <https://www.gnu.org/prep/standards/standards.html#Errors>
        println!(
            "{}:{}:{}: {}",
            err.filename, err.start, err.end, err.message
        );

        Output::Void
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib() {
        let prog = read_json("./json/fib.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Int(55));
    }

    #[test]
    fn print() {
        let prog = read_json("./json/print.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Void);
    }

    #[test]
    fn add1() {
        let prog = read_json("./json/add1.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Int(3));
    }

    #[test]
    fn add2() {
        let prog = read_json("./json/add2.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap_err();

        assert_eq!(res.message, "Cannot perform add operation");
    }

    #[test]
    fn concat1() {
        let prog = read_json("./json/concat1.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Str(String::from("1abc")));
    }

    #[test]
    fn concat2() {
        let prog = read_json("./json/concat2.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Str(String::from("abc1")));
    }

    #[test]
    fn concat3() {
        let prog = read_json("./json/concat3.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Str(String::from("abcdef")));
    }

    #[test]
    fn sub1() {
        let prog = read_json("./json/sub1.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Int(8));
    }

    #[test]
    fn sub2() {
        let prog = read_json("./json/sub2.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap_err();

        assert_eq!(res.message, "Cannot perform sub operation");
    }

    #[test]
    fn mul1() {
        let prog = read_json("./json/mul1.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Int(4));
    }

    #[test]
    fn mul2() {
        let prog = read_json("./json/mul2.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap_err();

        assert_eq!(res.message, "Cannot perform mul operation");
    }

    #[test]
    fn div1() {
        let prog = read_json("./json/div1.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Int(5));
    }

    #[test]
    fn div2() {
        let prog = read_json("./json/div2.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap_err();

        assert_eq!(res.message, "Arithmetic error, dividing by zero");
    }

    #[test]
    fn div3() {
        let prog = read_json("./json/div3.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap_err();

        assert_eq!(res.message, "Cannot perform div operation");
    }

    #[test]
    fn eq1() {
        let prog = read_json("./json/eq1.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Bool(true));
    }

    #[test]
    fn eq2() {
        let prog = read_json("./json/eq2.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Bool(false));
    }

    #[test]
    fn neq1() {
        let prog = read_json("./json/neq1.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Bool(true));
    }

    #[test]
    fn neq2() {
        let prog = read_json("./json/neq2.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Bool(false));
    }

    #[test]
    fn gt1() {
        let prog = read_json("./json/gt1.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Bool(true));
    }

    #[test]
    fn gt2() {
        let prog = read_json("./json/gt2.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Bool(true));
    }

    #[test]
    fn gt3() {
        let prog = read_json("./json/gt3.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap_err();

        assert_eq!(res.message, "Cannot perform gt operation");
    }

    #[test]
    fn gt4() {
        let prog = read_json("./json/gt4.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap_err();

        assert_eq!(res.message, "Cannot perform gt operation");
    }

    #[test]
    fn rem1() {
        let prog = read_json("./json/rem1.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Int(0));
    }

    #[test]
    fn rem2() {
        let prog = read_json("./json/rem2.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap_err();

        assert_eq!(res.message, "Cannot perform rem operation");
    }

    #[test]
    fn and1() {
        let prog = read_json("./json/and1.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Int(2));
    }

    #[test]
    fn and2() {
        let prog = read_json("./json/and2.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Int(5));
    }

    #[test]
    fn and3() {
        let prog = read_json("./json/and3.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Bool(false));
    }

    #[test]
    fn if1() {
        let prog = read_json("./json/if1.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Str(String::from("ok")));
    }

    #[test]
    fn if2() {
        let prog = read_json("./json/if2.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Str(String::from("fail")));
    }

    #[test]
    fn if3() {
        let prog = read_json("./json/if3.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap_err();

        assert_eq!(
            res.message,
            "Condition expression not resolve to a boolean primitive"
        );
    }

    #[test]
    fn tuple() {
        let prog = read_json("./json/tuple.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(
            res,
            Output::Tuple((Box::new(Output::Int(1)), Box::new(Output::Int(2))))
        );
    }

    #[test]
    fn var() {
        let prog = read_json("./json/var.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Int(3));
    }

    #[test]
    fn first1() {
        let prog = read_json("./json/first1.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Int(1));
    }

    #[test]
    fn first2() {
        let prog = read_json("./json/first2.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap_err();

        assert_eq!(res.message, "Cannot access first of a non tuple argument");
    }

    #[test]
    fn second1() {
        let prog = read_json("./json/second1.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert_eq!(res, Output::Int(7));
    }

    #[test]
    fn second2() {
        let prog = read_json("./json/second2.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap_err();

        assert_eq!(res.message, "Cannot access second of a non tuple argument");
    }

    #[test]
    fn closure1() {
        let prog = read_json("./json/closure1.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap();

        assert!(matches!(res, Output::Closure(..)));
    }

    #[test]
    fn closure2() {
        let prog = read_json("./json/closure2.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap_err();

        assert_eq!(
            res.message,
            "Arguments declaration differs parameters declaration"
        );
    }

    #[test]
    fn closure3() {
        let prog = read_json("./json/closure3.json");
        let mut context = Context {
            outter: None,
            inner: HashMap::new(),
        };

        let res = eval(prog.expression, &mut context).unwrap_err();

        assert_eq!(res.message, "Calling a not callable");
    }
}
