use clause;
use std::vec::Vec;

trait Sat {
    fn new(variable_amount: usize, clauses: &mut Vec<& clause::TwoPointerClause>)-> Self;
    fn unit_propagation(&mut self)->bool;
    fn choose(&mut self, variable: usize, assignment: bool);
}

#[derive(Debug)]
struct SatInstance<'a> {
    assignments: &'a mut Vec<Option<bool>>,
    clauses: &'a mut Vec<&'a clause::TwoPointerClause>
}
