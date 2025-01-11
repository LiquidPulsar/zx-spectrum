use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{alpha1, digit1, multispace0, multispace1, one_of};
use nom::combinator::map;
use nom::error::{context, VerboseError};
use nom::multi::{many0, separated_list1};
use nom::sequence::{pair, preceded, separated_pair, terminated};
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Instr<'a> {
    Print(Vec<Expr<'a>>),
    Assign(Expr<'a>, Expr<'a>), // Assign(Ident, Expr)
    Input(Expr<'a>, Expr<'a>),  // Input(Expr, Ident)
    Rem,
}

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Ident(&'a str),
    Int(i64),
    Add(Box<Expr<'a>>, Box<Expr<'a>>),
    Sub(Box<Expr<'a>>, Box<Expr<'a>>),
    Mul(Box<Expr<'a>>, Box<Expr<'a>>),
    Div(Box<Expr<'a>>, Box<Expr<'a>>),
}

type ParseResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

impl Expr<'_> {
    fn parse_ident(s: &str) -> ParseResult<Expr> {
        map(alpha1, Expr::Ident)(s)
    }

    fn parse_atom(s: &str) -> ParseResult<Expr> {
        alt((
            Expr::parse_ident,
            map(digit1, |s: &str| Expr::Int(s.parse().unwrap())),
        ))(s)
    }

    fn parse_general<'a>(
        f: &dyn Fn(&str) -> ParseResult<Expr>,
        chars: &'static str,
        s: &'a str,
    ) -> ParseResult<'a, Expr<'a>> {
        let (s, expr) = f(s)?;
        let (s, exprs) = many0(pair(one_of(chars), f))(s)?;
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

    pub fn parse(s: &str) -> ParseResult<Expr> {
        Expr::parse_general(&Expr::parse_term, "+-", s)
    }

    fn parse_fn<'a>(op: char, acc: Expr<'a>, expr: Expr<'a>) -> Expr<'a> {
        let fun = match op {
            '+' => Expr::Add,
            '-' => Expr::Sub,
            '*' => Expr::Mul,
            '/' => Expr::Div,
            _ => unreachable!(),
        };
        fun(Box::new(acc), Box::new(expr))
    }
}

impl Instr<'_> {
    pub fn parse(s: &str) -> ParseResult<Instr> {
        // TODO: Make this fail if there is any remaining input
        terminated(
            alt((
                context(
                    "print statement",
                    preceded(
                        terminated(tag_no_case("print"), multispace1), // Terminated by a space
                        map(
                            separated_list1(terminated(tag(","), multispace0), Expr::parse),
                            Instr::Print,
                        ),
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
                context("rem statement", map(tag_no_case("rem"), |_| Instr::Rem)),
                context(
                    "input statement",
                    map(
                        preceded(
                            terminated(tag_no_case("input"), multispace1),
                            separated_pair(
                                terminated(Expr::parse, multispace0),
                                terminated(tag(","), multispace0),
                                Expr::parse,
                            ),
                        ),
                        |(expr1, expr2)| Instr::Input(expr1, expr2),
                    ),
                ),
            )),
            multispace0,
        )(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expr() {
        assert_eq!(Expr::parse("42"), Ok(("", Expr::Int(42),)),);
        assert_eq!(Expr::parse("x"), Ok(("", Expr::Ident("x"),)),);
    }

    #[test]
    fn test_parse_instr() {
        assert_eq!(
            Instr::parse("print 42"),
            Ok(("", Instr::Print(vec![Expr::Int(42)]),)),
        );
        assert_eq!(
            Instr::parse("let x = 42"),
            Ok(("", Instr::Assign(Expr::Ident("x"), Expr::Int(42)),)),
        );
        // Test malformed
        assert!(Instr::parse("let x =").is_err());
    }

    #[test]
    fn test_precedence() {
        assert_eq!(
            Expr::parse("1 + 2 * 3"),
            Ok((
                "",
                Expr::Add(
                    Box::new(Expr::Int(1)),
                    Box::new(Expr::Mul(Box::new(Expr::Int(2)), Box::new(Expr::Int(3))))
                )
            )),
        );
        assert_eq!(
            Expr::parse("1 * 2 + 3"),
            Ok((
                "",
                Expr::Add(
                    Box::new(Expr::Mul(Box::new(Expr::Int(1)), Box::new(Expr::Int(2)))),
                    Box::new(Expr::Int(3))
                )
            )),
        );
    }
}
