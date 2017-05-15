use clause;
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

    /// this method removes the clauses of the indices "clauses" from the list of clauses
    /// since this is in O(|self.clauses|) it is smart to remove multiple clauses at once
    fn remove_clauses(&mut self, clauses: HashSet<usize>);

    /// this method assigns the variable of index "variable" to the "assignment"
    /// e.g.    Some(true) means the variable evaluates to 1
    ///         None means the variable evaluates to "unassigned"
    fn choose(&mut self, variable: usize, assignment: Option<bool>);

    /// sets the variable which makes the clause unit to the expected value and
    /// returns the variable which was assigned and the bool value to which it was assigned
    fn chooseUnit(&mut self, clauseIndex: usize)->(usize, bool);

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
        self.clauses.push(clause);
    }

    fn remove_clauses(&mut self, clauses: HashSet<usize>) {
        panic!(); //TODO implement
    }

    fn choose(&mut self, variable: usize, assignment: Option<bool>) {
        panic!(); //TODO implement
    }

    fn chooseUnit(&mut self, clauseIndex: usize) -> (usize,bool) {
        panic!(); //TODO implement
    }

    fn sat_state(&mut self) -> FormulaState {
        panic!(); //TODO implement
    }
}

#[derive(Debug)]
pub struct FormulaInstance {
    assignments: Vec<Option<bool>>,
    clauses: Vec<clause::TwoPointerClause>,
}

pub enum FormulaState {
    Conflict(usize),
    Unit(usize),
    Else,
}
