use std::vec::Vec;

trait Literal {
    fn value(&self) -> usize;
    fn is_satisfied(&self, variables: &Vec<Option<bool>>) -> bool;
}

#[derive(Clone, Debug)]
pub enum SimpleLiteral {
    Positive(usize),
    Negative(usize),
}
