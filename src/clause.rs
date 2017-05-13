use literal;

pub trait Clause {
    fn new(&Vec<literal::SimpleLiteral>) -> Self;
    fn clause_state(&self, assignments: &mut Vec<Option<bool>>) -> ClauseState;
}

#[derive(Debug)]
pub struct TwoPointerClause {}

#[derive(Debug)]
pub enum ClauseState {
    Open,
    Unit(literal::SimpleLiteral),
    Satisfied,
    Filled,
}
