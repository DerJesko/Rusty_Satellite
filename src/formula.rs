use clause;
use literal;
use std::vec::Vec;
use std::collections::{LinkedList, HashSet};

pub trait Formula {
    /// this method creates a sat instance which contains a list of clauses and "variable_amount"
    /// of unassigned continually enumerated variables
    /// This has a few assumptions, please check them before using:
    /// - the variables mention in "clauses" are in the interval [0,variable_amount)
    /// - there are no empty clauses
    fn new(variable_amount: usize, clauses: Vec<clause::TwoPointerClause>) -> Self;

    /// this method adds a clause to the end of the list
    fn add_clause(&mut self, clause::TwoPointerClause);

    /// this method removes the clause of the index "remove_index" from the list of clauses
    fn remove_clauses(&mut self, remove_index:usize);

    /// returns true iff there are unassigned variables
    fn hasUnassignedVars(&mut self) -> bool;

    /// this method assigns the variable of index "variable" to the "assignment"
    /// e.g.    Some(true) means the variable evaluates to 1
    ///         None means the variable evaluates to "unassigned"
    fn choose(&mut self, variable: usize, assignment: Option<bool>);

    /// sets the variable which makes the clause unit to the expected value and
    /// returns the literal which was assigned
    /// this assumes the state of the clause is updated
    fn chooseUnit(&mut self, clauseIndex: usize)->literal::SimpleLiteral;

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
    fn sat_state(&mut self) -> FormulaState;
}

impl Formula for FormulaInstance {
    fn new(variable_amount: usize, clauses: Vec<clause::TwoPointerClause>) -> FormulaInstance {
        FormulaInstance {
            clauses: clauses,
            assignments: vec![None; variable_amount]
        }
    }

    fn add_clause(&mut self, clause: clause::TwoPointerClause) {
        self.clauses.push(clause);  //TODO: add clause only if it does not exist already
    }

    fn remove_clauses(&mut self, remove_index: usize) {
        self.clauses.remove(remove_index);
    }

    fn hasUnassignedVars(&mut self) -> bool{
        for elem in &self.assignments{
            if elem.is_none() {
                return true;
            }
        }
        return false;
    }

    fn choose(&mut self, variable: usize, assignment: Option<bool>) {
        self.assignments[variable] = assignment;
    }

    fn chooseUnit(&mut self, clauseIndex: usize) -> literal::SimpleLiteral {
        if let clause::ClauseState::Unit(literal_index) = self.clauses[clauseIndex].state {
            match self.clauses[clauseIndex].literals[literal_index] {
                literal::SimpleLiteral::Positive(variable_index) => {
                    self.assignments[variable_index] = Some(true);
                    return (literal::SimpleLiteral::Positive(variable_index));
                }
                literal::SimpleLiteral::Negative(variable_index) => {
                    self.assignments[variable_index] = Some(false);
                    return (literal::SimpleLiteral::Negative(variable_index));
                }
            }
        } else { panic!("You should not be here") }
    }

    fn sat_state(&mut self) -> FormulaState {
        panic!("still waiting for implementation"); //TODO implement
    }
}

#[derive(Debug)]
pub struct FormulaInstance {
    pub assignments: Vec<Option<bool>>,
    pub clauses: Vec<clause::TwoPointerClause>,
}

pub enum FormulaState {
    Conflict(usize),
    Unit(usize),
    Else,
}
