use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct State<'a> {
    pub vars: HashMap<&'a str, i64>,
    pub pc: usize,
}