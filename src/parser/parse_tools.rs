use nom::{
    character::complete::multispace0,
    error::VerboseError,
    sequence::{preceded, terminated},
    IResult,
};

pub type NomErr<'a> = VerboseError<&'a str>;
pub type ParseResult<'a, T> = IResult<&'a str, T, NomErr<'a>>;

pub(crate) fn with_whitespaces<'a, F, O>(f: F) -> impl FnMut(&'a str) -> ParseResult<'a, O>
where
    F: FnMut(&'a str) -> ParseResult<'a, O>,
{
    preceded(multispace0, terminated(f, multispace0))
}
