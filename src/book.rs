use crate::recipe::Recipe;

#[derive(Debug, Eq, PartialEq)]
pub struct Book {
    recipes: Vec<Recipe>,
}
impl Book {
    pub fn new() -> Self {
        Self {
            recipes: Vec::new(),
        }
    }
}
