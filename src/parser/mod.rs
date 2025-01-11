mod instr;
mod parser_tests;
mod integration_tests;
use nom::error::VerboseError;

pub use self::instr::{Instr, Expr};

pub fn parse_file(file: &str) -> Result<Vec<Instr>, nom::Err<VerboseError<&str>>> {
    let mut instrs = Vec::new();
    for line in file.lines() {
        let (_, new_instrs) = Instr::parse(line)?;
        instrs.extend(new_instrs);
    }
    Ok(instrs)
}