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

macro_rules! impl_from {
    ($($t:ty => $constructor:expr),*) => {
        $(
            impl From<$t> for Value<'_> {
                fn from(s: $t) -> Self {
                    $constructor(s)
                }
            }
        )*
    };
}

impl <'a> From<&'a str> for Value<'a> {
    fn from(s: &'a str) -> Self {
        Value::String(s)
    }
}

impl_from! {
    i64 => Value::Int,
    bool => Value::Bool,
    char => Value::Char
}