use nom::branch::alt;
use nom::bytes::complete::take_till;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1};
use nom::combinator::{cut, map};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::{pair, preceded, terminated};

use crate::parser::parse_tools::{ParseResult, with_whitespaces};

type BExpr<'a> = Box<Expr<'a>>;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'a> {
    Ident(&'a str),
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

// This function is unsafe because it modifies the input string in place
// TODO: is there ever a case where we actually want to unwind this in the parser?
// I.E. is there a case where we want to backtrack after modifying the string?
fn unsafe_lowercase_inplace(s: &str) -> &str {
    unsafe {
        let mutable_raw_ptr= s.as_ptr() as *mut u8;
        for i in 0..s.len() {
            *mutable_raw_ptr.add(i) = (*mutable_raw_ptr.add(i)).to_ascii_lowercase();
        }
    }
    s
}

impl Expr<'_> {
    pub(crate) fn parse_ident(s: &str) -> ParseResult<Expr> {
        map(alpha1, |i| Expr::Ident(unsafe_lowercase_inplace(i)))(s)
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
            map(digit1, |s: &str| Expr::Int(s.parse().unwrap())),
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
        parse_general!(Expr::parse_factor, s, "<=",">=","<>","<",">","=") // Parse 2-char operators first
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
