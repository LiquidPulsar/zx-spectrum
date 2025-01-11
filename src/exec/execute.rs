use anyhow::{anyhow, Result};

use super::{State, Value};
use crate::parser::{Expr, Instr};

pub fn execute(instrs: Vec<Instr>) -> Result<(), anyhow::Error> {
    let mut state = State::default();
    while state.pc < instrs.len() {
        if !instrs[state.pc].execute(&mut state)? {
            state.pc += 1;
        }
    }
    Ok(())
}

impl<'a> Instr<'a> {
    fn execute<'b>(&self, state: &mut State<'b>) -> Result<bool> // Did we jump?
    where
        'a: 'b,
    {
        match self {
            Instr::Print(exprs) => {
                println!(
                    "{}",
                    exprs
                        .iter()
                        .map(|expr| expr.eval(state))
                        .collect::<Result<Vec<_>>>()?
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            Instr::Assign(Expr::Ident(ident), expr) => {
                state.vars.insert(ident, expr.eval_to_int(state)?);
            }
            Instr::Assign(expr, _) => {
                return Err(anyhow!(
                    "(In assignment instr) Expected identifier, found: {:?}",
                    expr
                ))
            }
            Instr::Rem(_) => {}
            Instr::Input(expr1, Expr::Ident(ident)) => {
                if let Some(expr) = expr1 {
                    println!("{}", expr.eval(state)?);
                }
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                // TODO: Impl "CONTINUE"
                if input.trim_end() == "STOP" {
                    return Err(anyhow!("Program stopped by user"));
                }
                let input = input.trim_end().parse::<i64>()?;
                state.vars.insert(ident, input);
            }
            Instr::Input(_, expr) => {
                return Err(anyhow!(
                    "(In input instr) Expected identifier, found: {:?}",
                    expr
                ))
            }
            Instr::Goto(pc) => {
                state.pc = (*pc / 10) - 1;
                return Ok(true);
            } // Convert from line number to 0-based index. Non-10s digits are ignored.
            Instr::Clear => print!("\x1B[2J\x1B[1;1H"), // ANSI escape codes to clear the screen and move the cursor to the top-left corner
            Instr::IfThen(expr, if_true) => match expr.eval(state)? {
                Value::Bool(true) => return if_true.execute(state),
                Value::Bool(false) => {}
                _ => return Err(anyhow!("Expected boolean, found: {:?}", expr)),
            },
            Instr::Multi(instrs) => {
                for instr in instrs {
                    if instr.execute(state)? {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    }
}

impl Expr<'_> {
    fn eval(&self, state: &State) -> Result<Value> {
        match self {
            Expr::Ident(ident) => Ok(state.get_var(ident)?.into()),
            Expr::Int(i) => Ok((*i).into()),
            Expr::Add(_, _) | Expr::Sub(_, _) | Expr::Mul(_, _) | Expr::Div(_, _) => {
                Ok(self.eval_to_int(state)?.into())
            }
            Expr::String(s) => Ok(Value::String(s)),
            Expr::Gt(expr1, expr2) => {
                Ok((expr1.eval_to_int(state)? > expr2.eval_to_int(state)?).into())
            }
            Expr::Lt(expr1, expr2) => {
                Ok((expr1.eval_to_int(state)? < expr2.eval_to_int(state)?).into())
            }
            Expr::Eq(expr1, expr2) => {
                Ok((expr1.eval_to_int(state)? == expr2.eval_to_int(state)?).into())
            }
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
            Expr::Gt(_, _) | Expr::Lt(_, _) | Expr::Eq(_, _) => {
                Err(anyhow!("Expected integer, found comparison: {:?}", self))
            }
        }
    }
}
