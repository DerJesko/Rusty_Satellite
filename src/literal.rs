use std::vec::Vec;
use std::fmt;

pub trait Literal {
    fn value(&self) -> usize;
    fn is_satisfied(&self, variables: &Vec<Option<bool>>) -> bool;
}

impl Literal for SimpleLiteral {

    fn value(&self) -> usize {
        match *self {
            SimpleLiteral::Positive(literal_index) => literal_index,
            SimpleLiteral::Negative(literal_index) => literal_index
        }
    }

    fn is_satisfied(&self, variables: &Vec<Option<bool>>) -> bool {
        match *self {
            SimpleLiteral::Positive(literal_index) => if let Some(assigned_value) = variables[literal_index] {
                assigned_value
            } else { false },
            SimpleLiteral::Negative(literal_index) => if let Some(assigned_value) = variables[literal_index] {
                !assigned_value
            } else { false }
        }
    }
}

impl fmt::Debug for SimpleLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SimpleLiteral::Positive(literal_index) => write!(f, "{}", literal_index),
            SimpleLiteral::Negative(literal_index) => write!(f, "-{}", literal_index)
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum SimpleLiteral {
    Positive(usize),
    Negative(usize),
}
