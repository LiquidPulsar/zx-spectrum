use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::{alpha1, char, digit1, multispace0, multispace1, one_of};
use nom::combinator::{all_consuming, cut, map, opt, rest, verify};
use nom::error::context;
use nom::multi::{many0, separated_list1};
use nom::sequence::{pair, preceded, separated_pair, terminated, tuple};

use crate::parser::expr::Expr;
use crate::parser::parse_tools::{with_whitespaces, ParseResult};

use super::parse_tools::ident;

// TODO: Add support for string vars, denoted with training $
#[derive(Debug, PartialEq, Clone)]
pub enum Instr<'a> {
    Print(Option<Expr<'a>>, Vec<(char,Expr<'a>)>, Option<char>),              // Print(first_expr, rest, last)
    Assign(Expr<'a>, Expr<'a>),        // Assign(Ident, Expr)
    Input(Option<Expr<'a>>, Expr<'a>), // Input(Expr, Ident)
    Rem(&'a str),
    Goto(usize),
    Clear,
    IfThen(Expr<'a>, Box<Instr<'a>>),
    Multi(Vec<Instr<'a>>),
    For(Expr<'a>, Expr<'a>, Expr<'a>, Expr<'a>), // For(ident, start, end, step)
    Next(Expr<'a>),
}

impl Instr<'_> {
    pub fn parse_prefixed(s: &str) -> ParseResult<Instr> {
        preceded(
            context("Prefixed line", pair(digit1, multispace1)),
            Instr::parse,
        )(s)
    }

    fn parse_print(s: &str) -> ParseResult<Instr> {
        alt((
            preceded(
                // Cut to avoid needless backtracking after committing
                // Cannot cut the multispace1 because we want to fall to the empty print statement case. Could refactor to avoid this?
                terminated(
                    tag_no_case("print"),
                    context("Space needed after print statement", multispace1),
                ), // Terminated by a space
                // TODO: Add the ; to the list of separators, and make it stick the elements together
                // TODO: Add functionality for trailing separators to stop the newline from being printed
                // TODO: Trailing '
                cut(map(
                    tuple((Expr::parse, many0(pair(with_whitespaces(one_of(",;")), Expr::parse)), opt(with_whitespaces(one_of(",;"))))),
                    |(first_expr, rest, last)| Instr::Print(Some(first_expr), rest, last)
                )),
            ),
            map(tag_no_case("print"), |_| Instr::Print(None, vec![], None)),
        ))(s)
    }

    fn parse_if_then(s: &str) -> ParseResult<Instr> {
        map(
            preceded(
                terminated(tag_no_case("if"), multispace1),
                cut(pair(
                    Expr::parse,
                    preceded(with_whitespaces(tag_no_case("then")), Instr::parse),
                )),
            ),
            |(expr, instrs)| Instr::IfThen(expr, Box::new(instrs)),
        )(s)
    }

    fn parse_input(s: &str) -> ParseResult<Instr> {
        map(
            preceded(
                terminated(tag_no_case("input"), multispace1),
                cut(pair(
                    Expr::parse,
                    opt(preceded(with_whitespaces(char(',')), Expr::parse)),
                )),
            ),
            |(expr1, expr2)| match expr2 {
                None => Instr::Input(None, expr1),
                Some(val) => Instr::Input(Some(expr1), val),
            },
        )(s)
    }

    fn parse_inner_assign(s: &str) -> ParseResult<(Expr, Expr)> {
        separated_pair(Expr::parse_ident, with_whitespaces(char('=')), Expr::parse)(s)
    }

    fn parse_assign(s: &str) -> ParseResult<Instr> {
        preceded(
            terminated(tag_no_case("let"), multispace1),
            cut(map(Instr::parse_inner_assign, |(ident, expr)| {
                Instr::Assign(ident, expr)
            })),
        )(s)
    }

    fn parse_name_as_ident(s: &str) -> ParseResult<Expr> {
        map(verify(alpha1, |x: &str| x.len() == 1), ident)(s)
    }

    // TODO: For loop can appear as part of single-line command, should be able to function as such
    fn parse_for(s: &str) -> ParseResult<Instr> {
        map(
            preceded(
                terminated(tag_no_case("for"), multispace1),
                cut(pair(
                    separated_pair(Instr::parse_name_as_ident, with_whitespaces(char('=')), Expr::parse), // TODO: make this only accept single letter identifiers set to ints
                    preceded(
                        with_whitespaces(tag_no_case("to")),
                        cut(pair(
                            Expr::parse, // TODO: make this only accept ints
                            opt(preceded(
                                with_whitespaces(tag_no_case("step")),
                                Expr::parse,
                            )),
                        )),
                    ),
                )),
            ),
            |((ident, start), (end, step))| Instr::For(ident, start, end, step.unwrap_or(Expr::Int(1))),
        )(s)
    }

    fn parse_next(s: &str) -> ParseResult<Instr> {
        map(
            preceded(
                terminated(tag_no_case("next"), multispace1),
                cut(Instr::parse_name_as_ident),
            ),
            Instr::Next,
        )(s)
    }

    pub fn parse_inner(s: &str) -> ParseResult<Instr> {
        alt((
            context("print statement", Instr::parse_print),
            context("let statement", Instr::parse_assign),
            context(
                "rem statement",
                map(
                    preceded(terminated(tag_no_case("rem"), opt(char(' '))), rest),
                    Instr::Rem,
                ),
            ),
            context("input statement", Instr::parse_input),
            context(
                "goto statement",
                map(preceded(tag_no_case("go to "), digit1), |x: &str| {
                    Instr::Goto(x.parse().expect("Line no. too large, this is insanity"))
                    // TODO: Handle error
                }),
            ),
            context("cls statement", map(tag_no_case("cls"), |_| Instr::Clear)),
            context("if then statement", Instr::parse_if_then),
            context(
                "stop statement",
                map(tag_no_case("stop"), |_| Instr::Goto(99999999999)),
            ),
            context("for loop", Instr::parse_for),
            context("next statement", Instr::parse_next),
        ))(s)
    }

    pub fn parse(s: &str) -> ParseResult<Instr> {
        let (s, res) = all_consuming(separated_list1(
            with_whitespaces(char(':')),
            terminated(Instr::parse_inner, multispace0),
        ))(s)?;

        Ok((
            s,
            if res.len() == 1 {
                res[0].clone()
            } else {
                Instr::Multi(res)
            },
        ))
    }
}
