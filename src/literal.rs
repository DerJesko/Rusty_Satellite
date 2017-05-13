use std::vec::Vec;

pub trait Literal {
    fn value(&self) -> usize;
    fn is_satisfied(&self, variables: &Vec<Option<bool>>) -> bool;
}

#[derive(Clone, Debug)]
pub enum SimpleLiteral {
    Positive(usize),
    Negative(usize),
}

#[derive(Clone, Debug)]
pub enum Asignment {
    True(usize),
    False(usize)
}
