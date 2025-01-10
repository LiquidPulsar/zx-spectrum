use crate::parser::{Instr, Expr};

use super::State;

pub fn execute(instrs: Vec<Instr>) -> Result<(), anyhow::Error> {
    let mut state = State::default();
    for instr in instrs {
        instr.execute(&mut state)?;
    }
    Ok(())
}

impl <'a> Instr<'a> {
    fn execute <'b> (&self, state: &mut State<'b>) -> Result<(), anyhow::Error> where 'a: 'b {
        match self {
            Instr::Print(ident) => {
                println!("{}", ident.eval(state));
            },
            Instr::Assign(Expr::Ident(ident), expr) => {
                state.vars.insert(ident, expr.eval(state));
            }
            _ => return Err(anyhow::anyhow!("Invalid instruction")),
        }
        Ok(())
    }
}

impl Expr<'_> {
    fn eval(&self, state: &State) -> i64 {
        match self {
            Expr::Ident(ident) => *state.vars.get(ident).unwrap(),
            Expr::Int(i) => *i,
        }
    }
}