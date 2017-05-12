use literal;

trait Clause {
    fn new(&Vec<literal::SimpleLiteral>) -> Self;
    fn clause_state(&self, assignments: &mut Vec<Option<bool>>) -> ClauseState;
}

#[derive(Debug)]
pub struct TwoPointerClause {

}

#[derive(Debug)]
enum ClauseState<'a>{
    Open,
    Unit(&'a literal::SimpleLiteral),
    Satisfied,
    Filled
}