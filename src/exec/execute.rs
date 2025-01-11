use anyhow::{anyhow, Result};

use crate::parser::{Expr, Instr};
use super::{State, Value};

pub fn execute(instrs: Vec<Instr>) -> Result<(), anyhow::Error> {
    let mut state = State::default();
    for instr in instrs {
        instr.execute(&mut state)?;
    }
    Ok(())
}

impl<'a> Instr<'a> {
    fn execute<'b>(&self, state: &mut State<'b>) -> Result<()>
    where
        'a: 'b,
    {
        match self {
            Instr::Print(exprs) => {
                println!(
                    "{}",
                    exprs
                        .iter()
                        .map(|expr| expr.eval(state)).collect::<Result<Vec<_>>>()?
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            Instr::Assign(Expr::Ident(ident), expr) => {
                state.vars.insert(ident, expr.eval_to_int(state)?);
            }
            Instr::Rem(_) => {}
            Instr::Input(expr1, Expr::Ident(ident)) => {
                println!("{}", expr1.eval(state)?);
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
    fn eval(&self, state: &State) -> Result<Value> {
        match self {
            Expr::Ident(ident) => Ok(state.get_var(ident)?.into()),
            Expr::Int(i) => Ok((*i).into()),
            Expr::Add(_, _) | Expr::Sub(_, _) | Expr::Mul(_, _) | Expr::Div(_, _) => Ok(self.eval_to_int(state)?.into()),
            Expr::String(s) => Ok(Value::String(s)),
        }
    }

    fn eval_to_int(&self, state: &State) -> Result<i64> {
        match self {
            Expr::Ident(ident) => Ok(state.get_var(ident)?),
            Expr::Int(i) => Ok(*i),
            Expr::Add(expr1, expr2) => Ok(expr1.eval_to_int(state)? + expr2.eval_to_int(state)?),
            Expr::Sub(expr1, expr2) => Ok(expr1.eval_to_int(state)? - expr2.eval_to_int(state)?),
            Expr::Mul(expr1, expr2) => Ok(expr1.eval_to_int(state)? * expr2.eval_to_int(state)?),
            Expr::Div(expr1, expr2) => Ok(expr1.eval_to_int(state)? / expr2.eval_to_int(state)?),
            Expr::String(s) => Err(anyhow!("Expected integer, found string: {}", s)),
        }
    }
}
