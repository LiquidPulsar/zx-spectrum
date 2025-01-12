use anyhow::{anyhow, Result};
use std::collections::HashMap;

use crate::parser::LowerCase;

#[derive(Debug, Default)]
pub struct State<'a> {
    pub vars: HashMap<LowerCase<'a>, i64>,
    pub pc: usize,
}

impl State<'_> {
    pub fn get_var(&self, ident: &LowerCase) -> Result<i64> {
        self.vars
            .get(ident)
            .ok_or(anyhow!("NameError: {}", ident))
            .copied()
    }
}
