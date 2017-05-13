use clause;
use std::vec::Vec;
use std::collections::LinkedList;

trait Formula {
    /// this method creates a sat instance which contains a list of clauses and "variable_amount"
    /// of unassigned continually enumerated variables
    fn new(variable_amount: usize, clauses: &mut LinkedList<&clause::TwoPointerClause>) -> Self;

    /// this method adds a clause to the end of the list
    fn add_clause(&mut self, &mut clause::TwoPointerClause);

    /// this method removes the clause of index "clause_index" from the list of clauses
    fn remove_clause(&mut self, clause_index: usize);

    /// this method assigns the variable of index "variable" to the "assignment"
    /// e.g.    Some(true) means the variable evaluates to 1
    ///         None means the variable evaluates to "unassigned"
    fn choose(&mut self, variable: usize, assignment: Option<bool>);

    /// this method returns the current state of the sat instance
    /// the priority is: Conflict > Unit > Else
    ///
    /// meaning: if there currently is a conflict it will return Conflict(x),
    /// wherein x the index of the clause is which caused the conflict.
    ///
    /// else if there currently is a unit clause it will return Unit(x),
    /// wherein x the index of a unit clause is.
    ///
    /// else it will return Else
    fn sat_state(&self) -> FormulaState;
}

#[derive(Debug)]
pub struct FormulaInstance<'a> {
    assignments: &'a mut Vec<Option<bool>>,
    clauses: &'a mut LinkedList<&'a clause::TwoPointerClause>,
}

enum FormulaState {
    Conflict(usize),
    Unit(usize),
    Else,
}
