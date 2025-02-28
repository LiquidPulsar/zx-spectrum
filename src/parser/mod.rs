mod expr;
mod instr;
mod integration_tests;
mod parse_tools;
mod parser_tests;
use parse_tools::NomErr;
mod lower;

pub use lower::LowerCase;
pub use expr::Expr;
pub use instr::Instr;

pub fn parse_file(file: &str, prefixed: bool) -> Result<Vec<Instr>, nom::Err<NomErr>> {
    let parse_fn = if prefixed {
        Instr::parse_prefixed
    } else {
        Instr::parse
    };
    file.lines() 
        .map(|line| parse_fn(line).map(|(_, res)| res))
        .collect()
}
