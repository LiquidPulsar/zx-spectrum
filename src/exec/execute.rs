use crate::parser::{Expr, Instr};

use super::State;

pub fn execute(instrs: Vec<Instr>) -> Result<(), anyhow::Error> {
    let mut state = State::default();
    for instr in instrs {
        instr.execute(&mut state)?;
    }
    Ok(())
}

impl<'a> Instr<'a> {
    fn execute<'b>(&self, state: &mut State<'b>) -> Result<(), anyhow::Error>
    where
        'a: 'b,
    {
        match self {
            Instr::Print(exprs) => {
                println!(
                    "{}",
                    exprs
                        .iter()
                        .map(|expr| expr.eval(state).to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            Instr::Assign(Expr::Ident(ident), expr) => {
                state.vars.insert(ident, expr.eval(state));
            }
            Instr::Rem => {}
            Instr::Input(expr1, Expr::Ident(ident)) => {
                println!("{}", expr1.eval(state));
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                let input = input.trim().parse::<i64>()?;
                state.vars.insert(ident, input);
            }
            _ => return Err(anyhow::anyhow!("Invalid instruction: {:?}", self)),
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
