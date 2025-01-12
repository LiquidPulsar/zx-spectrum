use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::{char, digit1, multispace0, multispace1};
use nom::combinator::{all_consuming, cut, map, opt, rest};
use nom::error::context;
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded, separated_pair, terminated};

use crate::parser::expr::Expr;
use crate::parser::parse_tools::{with_whitespaces, ParseResult};

#[derive(Debug, PartialEq, Clone)]
pub enum Instr<'a> {
    Print(Vec<Expr<'a>>),
    Assign(Expr<'a>, Expr<'a>),        // Assign(Ident, Expr)
    Input(Option<Expr<'a>>, Expr<'a>), // Input(Expr, Ident)
    Rem(&'a str),
    Goto(usize),
    Clear,
    IfThen(Expr<'a>, Box<Instr<'a>>),
    Multi(Vec<Instr<'a>>),
}

impl<'a> Instr<'a> {
    pub fn parse_prefixed(s: &'a str) -> ParseResult<'a, Instr> {
        preceded(
            context("Prefixed line", pair(digit1, multispace1)),
            Instr::parse,
        )(s)
    }

    pub fn parse(s: &'a str) -> ParseResult<'a, Instr> {
        let (s, res) = all_consuming(separated_list1(
            with_whitespaces(char(':')),
            terminated(
                alt((
                    context(
                        "print statement",
                        preceded(
                            // Cut to avoid needless backtracking after committing
                            // Cannot cut the multispace1 because we want to fall to the empty print statement case. Could refactor to avoid this?
                            terminated(
                                tag_no_case("print"),
                                context("Space needed after print statement", multispace1),
                            ), // Terminated by a space
                            // TODO: Add the ; to the list of separators, and make it stick the elements together
                            // TODO: Add functionality for trailing separators to stop the newline from being printed
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
                                cut(pair(
                                    Expr::parse,
                                    opt(preceded(with_whitespaces(char(',')), Expr::parse)),
                                )),
                            ),
                            |(expr1, expr2)| match expr2 {
                                None => Instr::Input(None, expr1),
                                Some(val) => Instr::Input(Some(expr1), val),
                            },
                        ),
                    ),
                    context(
                        "goto statement",
                        map(preceded(tag_no_case("go to "), digit1), |x: &str| {
                            Instr::Goto(x.parse().expect("Line no. too large, this is insanity"))
                            // TODO: Handle error
                        }),
                    ),
                    context("cls statement", map(tag_no_case("cls"), |_| Instr::Clear)),
                    context(
                        "if then statement",
                        map(
                            preceded(
                                terminated(tag_no_case("if"), multispace1),
                                cut(pair(
                                    Expr::parse,
                                    preceded(with_whitespaces(tag_no_case("then")), Instr::parse),
                                )),
                            ),
                            |(expr, instrs)| Instr::IfThen(expr, Box::new(instrs)),
                        ),
                    ),
                    context(
                        "stop statement",
                        map(tag_no_case("stop"), |_| Instr::Goto(99999999999)),
                    ),
                )),
                multispace0,
            ),
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
