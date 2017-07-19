use clause::{Clause, TwoPointerClause, ClauseState};
use std::vec::Vec;
use std::collections::HashSet;

pub trait Formula {
    /// this method creates a sat instance which contains a list of clauses and "variable_amount"
    /// of unassigned continually enumerated variables
    /// This has a few assumptions, please check them before using:
    /// - the variables mention in "clauses" are in the interval [0,variable_amount)
    /// - there are no empty clauses
    fn new(variable_amount: usize, clauses: HashSet<TwoPointerClause>) -> Self;

    /// this method adds a clause to the end of the list
    fn add_clause(&mut self, clause: TwoPointerClause);

    /// this method removes the clause of the index "remove_index" from the list of clauses
    fn remove_clause(&mut self, clause_to_remove:&TwoPointerClause);

    /// returns true if there are unassigned variables
    fn hasUnassignedVars(&mut self) -> bool;

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
    fn form_state(&mut self) -> FormulaState;
}

impl Formula for FormulaInstance {
    fn new(variable_amount: usize, clauses: HashSet<TwoPointerClause>) -> FormulaInstance {
        FormulaInstance {
            clauses: clauses,
            assignments: vec![None; variable_amount]
        }
    }

    fn add_clause(&mut self, clause: TwoPointerClause) {
        self.clauses.insert(clause);
    }

    fn remove_clause(&mut self, clause_to_remove: &TwoPointerClause) {
        self.clauses.remove(clause_to_remove);
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



    fn form_state(&mut self) -> FormulaState {
        let mut newSet: HashSet<TwoPointerClause> = HashSet::with_capacity(self.clauses.len());
        let mut return_clause:TwoPointerClause = TwoPointerClause::new(vec![]);
        let mut return_clause_is_conflict: bool = false;
        let mut return_clause_is_unit: bool = false;
        let mut return_clause_is: bool = false;
        
        for mut clause in self.clauses.drain() {
            clause.update_clause_state(&self.assignments);
            match clause.state {
                ClauseState::Open | ClauseState::Satisfied => {
                    if !return_clause_is{
                        return_clause = clause.clone();
                        return_clause_is = true;
                    }
                },
                ClauseState::Unit(_) => {
                    if !return_clause_is_unit{
                        return_clause = clause.clone();
                        return_clause_is = true;
                        return_clause_is_unit = true;
                    }
                },
                ClauseState::Unsatisfiable => {
                    if !return_clause_is_conflict{
                        return_clause = clause.clone();
                        return_clause_is = true;
                        return_clause_is_unit = true;
                        return_clause_is_conflict = true;
                    }
                }
            }
            newSet.insert(clause);
        }
        self.clauses = newSet;
        return if return_clause_is_conflict {FormulaState::Conflict(return_clause)} else if return_clause_is_unit {FormulaState::Unit(return_clause)} else {FormulaState::Else};
    }
}

#[derive(Debug, Clone)]
pub struct FormulaInstance {
    pub assignments: Vec<Option<bool>>,
    pub clauses: HashSet<TwoPointerClause>,
}


#[derive(Debug, Clone)]
pub enum FormulaState {
    Conflict(TwoPointerClause),
    Unit(TwoPointerClause),
    Else,
}
