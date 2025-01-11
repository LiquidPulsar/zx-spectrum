use anyhow::{anyhow, Result};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct State<'a> {
    pub vars: HashMap<&'a str, i64>,
    pub pc: usize,
}

impl <'a> State<'a> {
    pub fn get_var(&self, ident: &str) -> Result<i64> {
        self.vars.get(ident).ok_or(anyhow!("NameError: {}", ident)).copied()
    }
}