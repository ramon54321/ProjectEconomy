use std::fmt::Debug;

#[derive(Eq, PartialEq)]
pub struct Recipe {
    from: String,
    to: String,
}

impl Debug for Recipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}
