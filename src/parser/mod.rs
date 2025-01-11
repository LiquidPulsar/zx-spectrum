mod instr;
mod parser_tests;
mod integration_tests;
use instr::NomErr;

pub use self::instr::{Instr, Expr};

pub fn parse_file(file: &str, prefixed: bool) -> Result<Vec<Instr>, nom::Err<NomErr>> {
    let mut instrs = Vec::new();
    let parse_fn = if prefixed {
        Instr::parse_prefixed
    } else {
        Instr::parse
    };
    for line in file.lines() {
        let (_, new_instrs) = parse_fn(line)?;
        instrs.push(new_instrs);
    }
    Ok(instrs)
}