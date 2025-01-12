use nom::{
    character::complete::multispace0,
    error::VerboseError,
    sequence::{preceded, terminated},
    IResult,
};

use super::{Expr, LowerCase};

pub type NomErr<'a> = VerboseError<&'a str>;
pub type ParseResult<'a, T> = IResult<&'a str, T, NomErr<'a>>;

pub(crate) fn with_whitespaces<'a, F, O>(f: F) -> impl FnMut(&'a str) -> ParseResult<'a, O>
where
    F: FnMut(&'a str) -> ParseResult<'a, O>,
{
    preceded(multispace0, terminated(f, multispace0))
}

pub fn ident<'a>(s: &'a str) -> Expr<'a> {
    Expr::Ident(LowerCase(s))
}