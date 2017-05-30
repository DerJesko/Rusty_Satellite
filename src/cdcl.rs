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
    
    fn conflictAnalysis(&mut self, level:usize) -> usize{
        return 0;
    }
    
    /// adds the clause to the own formula and notifies the other threads that a new clause was found
    fn foundNewClause(&mut self, clause:TwoPointerClause){
        //add for own formula
        //share with other threads
    }
    
    /// checks if the channels received new clauses from other threads and adds them
    fn checkForNewClauses(&mut self){
        //check channels for new clauses and add them
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
            
            if self.unitPropagation(level).is_none() {  //backtracking (some failure)
            
            }
        }

        return true;
    }

}
