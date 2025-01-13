use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_till;
use nom::character::complete::{alpha1, char, i64};
use nom::combinator::{cut, map};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::{pair, preceded, terminated};

use crate::parser::lower::LowerCase;
use crate::parser::parse_tools::{with_whitespaces, ParseResult};

use super::parse_tools::ident;

type BExpr<'a> = Box<Expr<'a>>;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'a> {
    Ident(LowerCase<'a>),
    Int(i64),
    String(&'a str),
    Add(BExpr<'a>, BExpr<'a>),
    Sub(BExpr<'a>, BExpr<'a>),
    Mul(BExpr<'a>, BExpr<'a>),
    Div(BExpr<'a>, BExpr<'a>),
    Gt(BExpr<'a>, BExpr<'a>),
    Lt(BExpr<'a>, BExpr<'a>),
    Eq(BExpr<'a>, BExpr<'a>),
    Ge(BExpr<'a>, BExpr<'a>),
    Le(BExpr<'a>, BExpr<'a>),
    Ne(BExpr<'a>, BExpr<'a>),
}

macro_rules! parse_general {
    ($func:expr, $s:expr, $($parser:expr),*) => {
        {
            // Other option, but uses a clone which I don't like
            // fold_many0(pair(with_whitespaces(alt(($(tag($parser)),*))), $func), move || expr.clone(), |acc, (op, expr)| Expr::parse_fn(op, acc, expr))(s)
            let (s, expr) = $func($s)?;
            let (s, exprs) = many0(pair(with_whitespaces(alt(($(tag($parser)),*))), $func))(s)?;
            Ok((
                s,
                exprs
                    .into_iter()
                    .fold(expr, |acc, (op, expr)| Expr::parse_fn(op, acc, expr)),
            ))
        }
    }
}

impl Expr<'_> {
    pub(crate) fn parse_ident(s: &str) -> ParseResult<Expr> {
        map(alpha1, ident)(s)
    }

    fn parse_atom(s: &str) -> ParseResult<Expr> {
        alt((
            preceded(
                char('('),
                context(
                    "Parsing bracketed expr",
                    cut(terminated(
                        with_whitespaces(Expr::parse),
                        context("closing paren", char(')')),
                    )),
                ),
            ), // Parentheses
            Expr::parse_ident,
            map(i64, Expr::Int),
            preceded(
                char('"'),
                terminated(map(take_till(|x| x == '\"'), Expr::String), char('"')),
            ),
        ))(s)
    }

    fn parse_term(s: &str) -> ParseResult<Expr> {
        parse_general!(Expr::parse_atom, s, "*", "/")
    }

    fn parse_factor(s: &str) -> ParseResult<Expr> {
        parse_general!(Expr::parse_term, s, "+", "-")
    }

    pub fn parse(s: &str) -> ParseResult<Expr> {
        parse_general!(Expr::parse_factor, s, "<=", ">=", "<>", "<", ">", "=") // Parse 2-char operators first
    }

    fn parse_fn<'a>(op: &str, acc: Expr<'a>, expr: Expr<'a>) -> Expr<'a> {
        let fun = match op {
            "+" => Expr::Add,
            "-" => Expr::Sub,
            "*" => Expr::Mul,
            "/" => Expr::Div,
            ">" => Expr::Gt,
            "<" => Expr::Lt,
            "=" => Expr::Eq,
            "<>" => Expr::Ne,
            ">=" => Expr::Ge,
            "<=" => Expr::Le,
            _ => unreachable!(),
        };
        fun(Box::new(acc), Box::new(expr))
    }
}
