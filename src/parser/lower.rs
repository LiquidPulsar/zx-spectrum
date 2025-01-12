use std::hash::{Hash, Hasher};
use std::fmt::{Formatter, Display, Result};

#[derive(Debug, Clone)]
pub struct LowerCase<'a>(pub &'a str);

impl PartialEq for LowerCase<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl Eq for LowerCase<'_> {}

impl Hash for LowerCase<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_lowercase().hash(state)
    }
}

impl Display for LowerCase<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}