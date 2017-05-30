use formula::*;
use clause::*;
use literal::*;
use std::vec::Vec;


pub trait CdCl{
    fn new(initialFormula: FormulaInstance)->Self;
    fn sat(&mut self)->bool;
}


pub struct CdClInstance{
    formula: FormulaInstance,
    stack: Vec<StackElem>
}

pub enum StackElem{
    Implied(SimpleLiteral, usize),
    Chosen(SimpleLiteral, usize)
}

impl CdClInstance{

    //makes unitPropagation until there is a conflict (returns the clauseIndex) else it returns None
    fn unitPropagation(&mut self, level:usize) -> Option<usize>{
        while true {
            match self.formula.sat_state() {
                FormulaState::Unit(clauseIndex) => {
                    self.stack.push(StackElem::Implied(self.formula.chooseUnit(clauseIndex), level));
                },
                FormulaState::Conflict(clauseIndex) => return Some(clauseIndex),
                FormulaState::Else => return None
            }
        }
        panic!("Fatal error in unit Propagation!");
        return None;
    }

    fn getUnassignedVariable(&mut self) -> usize{
        for i in 0..self.formula.assignments.len(){
            if self.formula.assignments[i]==None {
                return i;
            }
        }
        panic!("There are no unassigned variables");
        return 0;
    }
    
    /// finds possibly a new clause and adds it to the formula
    /// returns the level to which one should backtrack (None if the formula is unsatisfiable)
    fn conflictAnalysis(&mut self, level:usize) -> Option<usize>{
        //TODO-Sanny: implement
        //call foundNewClause()
        return None;
    }
    
    /// adds the clause to the own formula and notifies the other threads that a new clause was found
    fn foundNewClause(&mut self, clause:TwoPointerClause){
        self.formula.add_clause(clause);
        //share with other threads
    }
    
    /// checks if the channels received new clauses from other threads and adds them
    /// returns true if new clauses are found from other threads
    fn checkChannelsForNewClauses(&mut self) -> bool{
        //check channels for new clauses and add them
    }
    
    /// backtracks until the choice of the passed level
    fn backtrack(&mut self, level:usize){
        while !self.stack.is_empty() {
            match self.stack.pop().unwrap() {
                StackElem::Chosen(literal, currentLevel) => {
                    if currentLevel == level {            //backtrack until right level is found
                        let newLiteral:SimpleLiteral;
                        match literal {
                            SimpleLiteral::Negative(variableIndex) => {
                                newLiteral = SimpleLiteral::Positive(variableIndex);
                                self.formula.choose(variableIndex, Some(true));
                            },
                            SimpleLiteral::Positive(variableIndex) => {
                                newLiteral = SimpleLiteral::Negative(variableIndex);
                                self.formula.choose(variableIndex, Some(false));
                            }
                        }
                        self.stack.push(StackElem::Implied(newLiteral, level - 1));
                        break;
                    } else {
                        self.formula.choose(literal.value(), None);  //unassign chosen with wrong level
                    }
                },
                StackElem::Implied(literal, _) => {
                    self.formula.choose(literal.value(), None);  //unassign implied
                }
            }
        }
    }
    
}

impl CdCl for CdClInstance{
    fn new(initialFormula: FormulaInstance)->Self{
        return CdClInstance{formula: initialFormula, stack: vec![]};
    }

    fn sat(&mut self)->bool{

        //first unitPropagation
        if self.unitPropagation(0).is_none() {
            return false;
        }
        let mut level:usize = 0;
        
        while self.formula.hasUnassignedVars() {
            //unitPropagation already done
            let chosen:SimpleLiteral = SimpleLiteral::Positive(self.getUnassignedVariable());
            self.formula.choose(chosen.value(), Some(true));
            self.stack.push(StackElem::Chosen(chosen, level));
            level += 1;
    
            self.checkChannelsForNewClauses();
            if self.unitPropagation(level).is_none() {  //backtracking (some failure)
                let returnLevel = self.conflictAnalysis(level);
                if returnLevel.is_none() {
                    return false;
                }
                level = returnLevel.unwrap();
                self.backtrack(level);
                self.unitPropagation(level);
            }
        }

        return true;
    }

}
