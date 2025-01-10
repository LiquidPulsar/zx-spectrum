use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{alpha1, digit1, multispace0, multispace1};
use nom::combinator::map;
use nom::error::{context, VerboseError};
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

#[derive(Debug)]
pub enum Instr<'a> {
    Print(Expr<'a>),
    Assign(Expr<'a>, Expr<'a>), // Assign(Ident, Expr)
}

#[derive(Debug)]
pub enum Expr<'a> {
    Ident(&'a str),
    Int(i64),
    // Add(Box<Expr>, Box<Expr>),
}

impl Expr<'_> {
    fn parse_ident(s: &str) -> IResult<&str, Expr<'_>, VerboseError<&str>> {
        map(alpha1, Expr::Ident)(s)
    }
    pub fn parse(s: &str) -> IResult<&str, Expr, VerboseError<&str>> {
        alt((
            Expr::parse_ident,
            map(digit1, |s: &str| Expr::Int(s.parse().unwrap())),
        ))(s)
    }
}

impl Instr<'_> {
    pub fn parse(s: &str) -> IResult<&str, Instr, VerboseError<&str>> {
        terminated(
            alt((
                context(
                    "print statement",
                    preceded(
                        terminated(tag_no_case("print"), multispace1), // Terminated by a space
                        map(Expr::parse, Instr::Print),
                    ),
                ),
                context(
                    "let statement",
                    preceded(
                        terminated(tag_no_case("let"), multispace1),
                        map(
                            separated_pair(
                                terminated(Expr::parse_ident, multispace0),
                                terminated(tag("="), multispace0),
                                Expr::parse,
                            ),
                            |(ident, expr)| Instr::Assign(ident, expr),
                        ),
                    ),
                ),
            )),
            multispace0,
        )(s)
    }
}
