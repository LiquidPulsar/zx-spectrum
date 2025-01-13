use nom::{
    character::complete::multispace0,
    error::VerboseError,
    sequence::delimited,
    IResult,
};

use super::{Expr, LowerCase};

pub type NomErr<'a> = VerboseError<&'a str>;
pub type ParseResult<'a, T> = IResult<&'a str, T, NomErr<'a>>;

pub(crate) fn with_whitespaces<'a, F, O>(f: F) -> impl FnMut(&'a str) -> ParseResult<'a, O>
where
    F: FnMut(&'a str) -> ParseResult<'a, O>,
{
    delimited(multispace0, f, multispace0)
}

pub fn ident(s: &str) -> Expr {
    Expr::Ident(LowerCase(s))
}