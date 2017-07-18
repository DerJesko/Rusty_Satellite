use formula::*;
use clause::*;
use literal::*;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::vec::Vec;


pub trait CdCl{
    fn new(initialFormula: FormulaInstance, receiver:Option<Receiver<TwoPointerClause>>, senders:Vec<Sender<TwoPointerClause>>)->Self;
    fn sat(&mut self)->bool;
}


pub struct CdClInstance{
    formula: FormulaInstance,
    stack: Vec<StackElem>,
    receiver: Option<Receiver<TwoPointerClause>>,
    senders: Vec<Sender<TwoPointerClause>>,
    learntClause: TwoPointerClause
}

pub enum StackElem{
    Implied(SimpleLiteral, usize, TwoPointerClause),
    Chosen(SimpleLiteral, usize)
}


impl CdClInstance{

    fn unitPropagation(&mut self, level:usize) -> Option<TwoPointerClause>{
    //makes unitPropagation until there is a conflict (returns the clauseIndex) else it returns None
        while true {
            match self.formula.form_state() {
                FormulaState::Unit(clause) => {
                    self.stack.push(StackElem::Implied(clause.chooseUnit(&mut self.formula.assignments), level, clause));
                },
                FormulaState::Conflict(clause) => return Some(clause),
                FormulaState::Else => return None
            }
        }
        panic!("Fatal error in unit Propagation!");
        return None;
    }

    /// chooses the variable which should be tried next (in a Choose-step)
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
        //TODO: UNSAT?
        let mut clause: TwoPointerClause;
        if let FormulaState::Conflict(conflictClause) = self.formula.form_state() {
            clause = conflictClause.clone();
        } else {
            panic!("ConflictAnalysis without conflict?")
        }

        while self.numberOfAssignedVariables(&clause, level) != 1 {
            for l in clause.clone().literals {
                match self.getImpliedLiteralAtLevel(&clause, &l, level) {
                    Some(x) => {
                        clause = clause.resolute(&x);
                        break;
                    },
                    None => continue
                }
            }
        }

        let backLevel: usize;
        if clause.literals.len() == 1 {
            backLevel = 0;
        } else {
            backLevel = self.getSecondHighestDecisionLevel(&clause);
        }

        self.foundNewClause(clause);

        return Some(backLevel);
    }
    
    /// adds the clause to the own formula and notifies the other threads that a new clause was found
    fn foundNewClause(&mut self, clause:TwoPointerClause){
    
        for sender in &self.senders {
            if sender.send(clause.clone()).is_err() {
                panic!("Could not send clause!");
            }
        }

        self.learntClause = clause.clone();
        self.formula.add_clause(clause);  //can't be added before sending
    }
    
    /// checks if the channels received new clauses from other threads and adds them
    /// returns true if new clauses are found from other threads
    fn checkReceiverForNewClauses(&mut self) -> bool{
        
        let mut foundFormula = false;
        if self.receiver.is_none() {
            return foundFormula;
        }
        
        loop {
            let ref mut receiver = self.receiver.as_ref().unwrap();
            match receiver.try_recv() {
                Ok(clause) => {
                    self.formula.add_clause(clause);
                    foundFormula = true;
                },
                Err(err) => {
                    return foundFormula;
                }
            }
        }
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
                        self.stack.push(StackElem::Implied(newLiteral, level - 1, self.learntClause));
                        break;
                    } else {
                        self.formula.choose(literal.value(), None);  //unassign chosen with wrong level
                    }
                },
                StackElem::Implied(literal, _, _) => {
                    self.formula.choose(literal.value(), None);  //unassign implied
                }
            }
        }
    }

    fn getImpliedLiteralAtLevel(&self, clause: &TwoPointerClause, literal: &SimpleLiteral, level: usize) -> Option<StackElem> {
        if !clause.literals.contains(&literal) {
            return None;
        }

        for elem in self.stack {
            match elem {
                StackElem::Implied(literal, level, _) => return Some(elem),
                _ => continue
            }
        }

        None

    }

    fn assignedAt(&self, literal: &SimpleLiteral, level: usize) -> bool {
        for elem in self.stack {
            match elem {
                StackElem::Implied(literal, level, _) => return true, // TODO: does this work? same literal/level as the arguments?
                StackElem::Chosen(literal, level) => return true,
                _ => continue
            }
        }
        false
    }

    fn numberOfAssignedVariables(&self, clause: &TwoPointerClause, level: usize) -> usize {
        let mut count = 0;
        for l in clause.literals {
            if self.assignedAt(&l, level) {
                count += 1;
            }
        }
        count
    }

    pub fn getAntecedent(elem: &StackElem) -> Option<TwoPointerClause> {
        match *elem {
            StackElem::Implied(_, _, antecedent) => return Some(antecedent),
            _ => return None
        }
    }

    fn getSecondHighestDecisionLevel(&self, clause: &TwoPointerClause) -> usize {
        let mut highest = 0;
        let mut second = 0;
        let mut d = 0;

        for l in clause.literals {
            d = self.getDecisionLevel(l);
            if d > highest {
                highest = d;
                second = highest;
            }
        }

        second
    }

    fn getDecisionLevel(&self, literal: SimpleLiteral) -> usize {
        for elem in self.stack {
            match elem {
                StackElem::Chosen(lit, d) => {
                    if lit == literal {
                        return d;
                    }
                },
                StackElem::Implied(lit, d, _) => {
                    if lit == literal {
                        return d;
                    }
                }
            }
        }
        panic!("You should not be here!");
    }

}

impl CdCl for CdClInstance{
    
    /// Constructor
    fn new(initialFormula: FormulaInstance, receiver:Option<Receiver<TwoPointerClause>>, senders:Vec<Sender<TwoPointerClause>>)->Self{
        return CdClInstance{formula: initialFormula, stack: vec!(), receiver: receiver, senders: senders,
            learntClause: TwoPointerClause::new(vec!())};
    }

    /// checks wether the formula is satisfiable
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
    
            self.checkReceiverForNewClauses();
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
