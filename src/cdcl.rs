use formula::*;
use clause::*;
use literal::*;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::vec::Vec;
use self::rand::Rng;

extern crate rand;


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

#[derive (Clone, Debug, Hash, Eq, PartialEq)]
pub enum StackElem{
    Implied(SimpleLiteral, usize, TwoPointerClause),
    Chosen(SimpleLiteral, usize)
}


impl CdClInstance{
    
    ///makes unitPropagation until there is a conflict (returns the clauseIndex) else it returns None
    fn unitPropagation(&mut self, level:usize) -> Option<TwoPointerClause>{
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
    fn getUnassignedVariable(&mut self, random: &mut Rng) -> usize{
        /*let mut order : [usize; self.formula.assignments.len()];
        for i in 0..self.formula.assignments.len(){
            order[i]=i;
        }
        random.shuffle(&mut order);*/
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
    fn conflictAnalysis(&mut self, mut clause: TwoPointerClause, level:usize) -> Option<(TwoPointerClause,usize)>{
        //return Some((TwoPointerClause::new(vec![]), level-1));
    
        //check if Unsatisfiable
        let mut foundNonImplied = false;
        for elem in &self.stack {
            match *elem {
                StackElem::Implied(_,_,_) => continue,
                _ => {
                    foundNonImplied = true;
                    break;
                }
            }
        }
        if !foundNonImplied {
            return None;
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

        return Some((clause, backLevel));
        
    }
    
    /// adds the clause to the own formula and notifies the other threads that a new clause was found
    fn foundNewClause(&mut self, clausee:&TwoPointerClause){
    
        for sender in &self.senders {
            if sender.send(clausee.clone()).is_err() {
                panic!("Could not send clause!");
            }
        }

        self.formula.add_clause(clausee.clone());  //can't be added before sending
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
    fn backtrack(&mut self, level:usize, learntClause:TwoPointerClause){
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
                        self.stack.push(StackElem::Implied(newLiteral, level - 1, learntClause));
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

        for elem in &self.stack {
            match *elem {
                StackElem::Implied(ref lit, lvl, _) => {
                    if (*lit == *literal && lvl==level) {
                        return Some(elem.clone());
                    }
                },
                _ => continue
            }
        }

        None

    }

    fn assignedAt(&self, literal: &SimpleLiteral, level: usize) -> bool {
        for elem in &self.stack {
            match *elem {
                StackElem::Implied(ref lit, lvl, _) => {
                    if (*lit == *literal && lvl==level) {
                        return true
                    }
                },
                StackElem::Chosen(ref lit, lvl) => {
                    if (*lit == *literal && lvl==level) {
                        return true
                    }
                }
            }
        }
        false
    }

    fn numberOfAssignedVariables(&self, clause: &TwoPointerClause, level: usize) -> usize {
        let mut count = 0;
        for l in &clause.literals {
            if self.assignedAt(&l, level) {
                count += 1;
            }
        }
        count
    }

    pub fn getAntecedent(elem: &StackElem) -> Option<TwoPointerClause> {
        match *elem {
            StackElem::Implied(_, _, ref antecedent) => return Some(antecedent.clone()),
            _ => return None
        }
    }

    fn getSecondHighestDecisionLevel(&self, clause: &TwoPointerClause) -> usize {
        let mut highest = 0;
        let mut second = 0;
        let mut d = 0;

        for l in &clause.literals {
            d = self.getDecisionLevel(&l);
            if d > highest {
                highest = d;
                second = highest;
            }
        }

        second
    }

    fn getDecisionLevel(&self, literal: &SimpleLiteral) -> usize {
        for elem in &self.stack {
            match *elem {
                StackElem::Chosen(ref lit, d) => {
                    if *lit == *literal {
                        return d;
                    }
                },
                StackElem::Implied(ref lit, d, _) => {
                    if *lit == *literal {
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
        let mut random = rand::thread_rng();
        
        //first unitPropagation
        if !self.unitPropagation(0).is_none() {
            return false;
        }
        let mut level:usize = 0;
        
        while self.formula.hasUnassignedVars() {
            //unitPropagation already done
            let unassigned = self.getUnassignedVariable(&mut random);
            let chosen:SimpleLiteral;
            if random.gen() {
                chosen = SimpleLiteral::Positive(unassigned);
            } else {
                chosen = SimpleLiteral::Negative(unassigned);
            }
            self.formula.choose(chosen.value(), Some(true));
            self.stack.push(StackElem::Chosen(chosen, level));
            level += 1;
    
            self.checkReceiverForNewClauses();
            let conflict = self.unitPropagation(level);
            if !conflict.is_none() {  //backtracking (some failure)
                let result = self.conflictAnalysis(conflict.unwrap(), level);
                if result.is_none() {
                    return false;
                }
                let (newClause, level) = result.unwrap();
                self.foundNewClause(&newClause);
                self.backtrack(level, newClause);
                self.unitPropagation(level);
            }
        }

        return true;
    }

}
