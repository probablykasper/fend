use crate::num::bigrat::BigRat;
use std::fmt::{Debug, Error, Formatter};

#[derive(Clone)]
pub enum Expr {
    Num(BigRat),
    Ident(String),
    Parens(Box<Expr>),
    UnaryMinus(Box<Expr>),
    UnaryPlus(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Expr::Num(n) => write!(f, "{}", n)?,
            Expr::Ident(ident) => write!(f, "{}", ident)?,
            Expr::Parens(x) => write!(f, "({:?})", *x)?,
            Expr::UnaryMinus(x) => write!(f, "(-{:?})", *x)?,
            Expr::UnaryPlus(x) => write!(f, "(+{:?})", *x)?,
            Expr::Add(a, b) => write!(f, "({:?}+{:?})", *a, *b)?,
            Expr::Sub(a, b) => write!(f, "({:?}-{:?})", *a, *b)?,
            Expr::Mul(a, b) => write!(f, "({:?}*{:?})", *a, *b)?,
            Expr::Div(a, b) => write!(f, "({:?}/{:?})", *a, *b)?,
            Expr::Pow(a, b) => write!(f, "({:?}^{:?})", *a, *b)?,
            Expr::Apply(a, b) => write!(f, "({:?} {:?})", *a, *b)?,
        };
        Ok(())
    }
}

pub fn evaluate(expr: Expr) -> Result<BigRat, String> {
    Ok(match expr {
        Expr::Num(n) => n,
        Expr::Ident(ident) => resolve_identifier(ident.as_str())?,
        Expr::Parens(x) => evaluate(*x)?,
        Expr::UnaryMinus(x) => -evaluate(*x)?,
        Expr::UnaryPlus(x) => evaluate(*x)?,
        Expr::Add(a, b) => evaluate(*a)? + evaluate(*b)?,
        Expr::Sub(a, b) => evaluate(*a)? - evaluate(*b)?,
        Expr::Mul(a, b) => evaluate(*a)? * evaluate(*b)?,
        Expr::Div(a, b) => evaluate(*a)?.div(evaluate(*b)?)?,
        Expr::Pow(a, b) => evaluate(*a)?.pow(evaluate(*b)?)?,
        Expr::Apply(a, b) => evaluate(*a)? * evaluate(*b)?,
    })
}

fn resolve_identifier(ident: &str) -> Result<BigRat, String> {
    Ok(match ident {
        "pi" => BigRat::approx_pi(),
        _ => return Err(format!("Unknown identifier '{}'", ident)),
    })
}
