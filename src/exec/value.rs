use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Int(i64),
    Bool(bool),
    Char(char),
    String(&'a str),
}

impl Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Char(c) => write!(f, "{}", c),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}

impl <'a> From<&'a str> for Value<'a> {
    fn from(s: &'a str) -> Self {
        Value::String(s)
    }
}

impl <'a> From<i64> for Value<'a> {
    fn from(s: i64) -> Self {
        Value::Int(s)
    }
}

impl <'a> From<bool> for Value<'a> {
    fn from(s: bool) -> Self {
        Value::Bool(s)
    }
}

impl <'a> From<char> for Value<'a> {
    fn from(s: char) -> Self {
        Value::Char(s)
    }
}