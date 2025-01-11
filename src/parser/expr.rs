use nom::branch::alt;
use nom::bytes::complete::take_till;
use nom::character::complete::{alpha1, char, digit1, one_of};
use nom::combinator::{cut, map};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::{pair, preceded, terminated};

use crate::parser::parse_tools::{ParseResult, with_whitespaces};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'a> {
    Ident(&'a str),
    Int(i64),
    String(&'a str),
    Add(Box<Expr<'a>>, Box<Expr<'a>>),
    Sub(Box<Expr<'a>>, Box<Expr<'a>>),
    Mul(Box<Expr<'a>>, Box<Expr<'a>>),
    Div(Box<Expr<'a>>, Box<Expr<'a>>),
    Gt(Box<Expr<'a>>, Box<Expr<'a>>),
    Lt(Box<Expr<'a>>, Box<Expr<'a>>),
    Eq(Box<Expr<'a>>, Box<Expr<'a>>),
}

impl Expr<'_> {
    pub(crate) fn parse_ident(s: &str) -> ParseResult<Expr> {
        map(alpha1, Expr::Ident)(s)
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

    fn parse_general<'a>(
        f: &dyn Fn(&str) -> ParseResult<Expr>,
        chars: &'static str,
        s: &'a str,
    ) -> ParseResult<'a, Expr<'a>> {
        let (s, expr) = f(s)?;
        let (s, exprs) = many0(pair(with_whitespaces(one_of(chars)), f))(s)?;
        Ok((
            s,
            exprs
                .into_iter()
                .fold(expr, |acc, (op, expr)| Expr::parse_fn(op, acc, expr)),
        ))
    }

    fn parse_term(s: &str) -> ParseResult<Expr> {
        Expr::parse_general(&Expr::parse_atom, "*/", s)
    }

    fn parse_factor(s: &str) -> ParseResult<Expr> {
        Expr::parse_general(&Expr::parse_term, "+-", s)
    }

    pub fn parse(s: &str) -> ParseResult<Expr> {
        Expr::parse_general(&Expr::parse_factor, "<>=", s)
    }

    fn parse_fn<'a>(op: char, acc: Expr<'a>, expr: Expr<'a>) -> Expr<'a> {
        let fun = match op {
            '+' => Expr::Add,
            '-' => Expr::Sub,
            '*' => Expr::Mul,
            '/' => Expr::Div,
            '>' => Expr::Gt,
            '<' => Expr::Lt,
            '=' => Expr::Eq,
            _ => unreachable!(),
        };
        fun(Box::new(acc), Box::new(expr))
    }
}
