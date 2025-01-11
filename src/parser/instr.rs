use nom::branch::alt;
use nom::bytes::complete::{tag_no_case, take_till};
use nom::character::complete::{alpha1, char, digit1, multispace0, multispace1, one_of};
use nom::combinator::{all_consuming, cut, map, opt, rest};
use nom::error::{context, VerboseError};
use nom::multi::{many0, separated_list1};
use nom::sequence::{pair, preceded, separated_pair, terminated};
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Instr<'a> {
    Print(Vec<Expr<'a>>),
    Assign(Expr<'a>, Expr<'a>), // Assign(Ident, Expr)
    Input(Expr<'a>, Expr<'a>),  // Input(Expr, Ident)
    Rem(&'a str),
    Goto(usize),
}

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Ident(&'a str),
    Int(i64),
    String(&'a str),
    Add(Box<Expr<'a>>, Box<Expr<'a>>),
    Sub(Box<Expr<'a>>, Box<Expr<'a>>),
    Mul(Box<Expr<'a>>, Box<Expr<'a>>),
    Div(Box<Expr<'a>>, Box<Expr<'a>>),
}

pub type NomErr<'a> = VerboseError<&'a str>;
pub type ParseResult<'a, T> = IResult<&'a str, T, NomErr<'a>>;

impl Expr<'_> {
    fn parse_ident(s: &str) -> ParseResult<Expr> {
        map(alpha1, Expr::Ident)(s)
    }

    fn parse_atom(s: &str) -> ParseResult<Expr> {
        alt((
            preceded(char('('), context("Parsing bracketed expr", cut(terminated(with_whitespaces(Expr::parse), context("closing paren", char(')')))))), // Parentheses
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

fn with_whitespaces<'a, F, O>(f: F) -> impl FnMut(&'a str) -> ParseResult<'a, O>
where
    F: FnMut(&'a str) -> ParseResult<'a, O>,
{
    preceded(multispace0, terminated(f, multispace0))
}

impl Instr<'_> {
    pub fn parse(s: &str) -> ParseResult<Vec<Instr>> {
        all_consuming(separated_list1(with_whitespaces(char(':')), terminated(
            alt((
                context(
                    "print statement",
                    preceded(
                        // Cut to avoid needless backtracking after committing
                        // Cannot cut the multispace1 because we want to fall to the empty print statement case. Could refactor to avoid this?
                        terminated(tag_no_case("print"), context("Space needed after print statement", multispace1)), // Terminated by a space
                        cut(map(
                            separated_list1(with_whitespaces(char(',')), Expr::parse),
                            Instr::Print,
                        )),
                    ),
                ),
                context(
                    "print statement (empty)",
                    map(tag_no_case("print"), |_| Instr::Print(vec![])),
                ),
                context(
                    "let statement",
                    preceded(
                        terminated(tag_no_case("let"), multispace1),
                        cut(map(
                            separated_pair(
                                Expr::parse_ident,
                                with_whitespaces(char('=')),
                                Expr::parse,
                            ),
                            |(ident, expr)| Instr::Assign(ident, expr),
                        )),
                    ),
                ),
                context(
                    "rem statement",
                    map(
                        preceded(terminated(tag_no_case("rem"), opt(char(' '))), rest),
                        Instr::Rem,
                    ),
                ),
                context(
                    "input statement",
                    map(
                        preceded(
                            terminated(tag_no_case("input"), multispace1),
                            cut(separated_pair(Expr::parse, with_whitespaces(char(',')), Expr::parse)),
                        ),
                        |(expr1, expr2)| Instr::Input(expr1, expr2),
                    ),
                ),
                context(
                    "goto statement",
                    map(preceded(tag_no_case("go to "), digit1), |x: &str| {
                        Instr::Goto(x.parse().expect("Line no. too large, this is insanity")) // TODO: Handle error
                    }),
                ),
            )),
            multispace0,
        )))(s)
    }
}
