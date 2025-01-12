mod expr;
mod instr;
mod integration_tests;
mod parse_tools;
mod parser_tests;
use parse_tools::NomErr;

pub use self::expr::Expr;
pub use self::instr::Instr;

pub fn parse_file(file: &str, prefixed: bool) -> Result<Vec<Instr>, nom::Err<NomErr>> {
    let parse_fn = if prefixed {
        Instr::parse_prefixed
    } else {
        Instr::parse
    };
    file.lines()
        .map(|line| parse_fn(line).map(|(_, res)| res))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.into())
}
