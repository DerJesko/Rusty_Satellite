use formula::*;
use clause::*;
use literal::*;
use std::sync::mpsc::{Sender, Receiver};
use std::vec::Vec;
use self::rand::{Rng,ThreadRng};
use self::rand::{Rng,thread_rng,ThreadRng};
use std::cmp;
use std::collections::HashSet;

extern crate rand;


pub trait CdCl{
    fn new(initialFormula: FormulaInstance, receiver:Option<Receiver<TwoPointerClause>>, senders:Vec<Sender<TwoPointerClause>>)->Self;
    fn sat(&mut self)->bool;
}


pub struct CdClInstance{
    pub formula: FormulaInstance,
    stack: Vec<StackElem>,
    receiver: Option<Receiver<TwoPointerClause>>,
    senders: Vec<Sender<TwoPointerClause>>
}

#[derive (Clone, Debug, Hash, Eq, PartialEq)]
pub enum StackElem{
    Implied(SimpleLiteral, isize, TwoPointerClause),
    Chosen(SimpleLiteral, isize)
}


impl CdClInstance{
    
    ///makes unitPropagation until there is a conflict (returns the clauseIndex) else it returns None
    /// else means: either every variable is assigned or you have to choose one variable
    fn unitPropagation(&mut self, level:isize) -> Option<TwoPointerClause>{
        loop {
            match self.formula.form_state() {
                FormulaState::Unit(clause) => {
                    self.stack.push(StackElem::Implied(clause.chooseUnit(&mut self.formula.assignments), level, clause));
                },
                FormulaState::Conflict(clause) => return Some(clause),
                FormulaState::Else => return None
            }
        }
    }

    /// chooses the variable which should be tried next (in a Choose-step)
    fn getUnassignedVariable(&mut self, random: &mut ThreadRng) -> usize{
        let mut order = vec![0; self.formula.assignments.len()];
        for i in 0..self.formula.assignments.len(){
            order[i]=i;
        }
        random.shuffle(&mut order);
        for i in order{
            if self.formula.assignments[i]==None {
                return i;
            }
        }
        panic!("There are no unassigned variables");
        return 0;
    }
    
    /// finds possibly a new clause and adds it to the formula
    /// returns the level to which one should backtrack (None if the formula is unsatisfiable)
    fn conflictAnalysis(&mut self, mut clause: TwoPointerClause, level:isize) -> Option<(TwoPointerClause,isize)>{
        
    
        //check if unsatisfiable
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
        //return Some((clause, level));  //only DPLL
        
        
        //iterate trough all literals l in clause
            //look in stack for l
            //--if l.level == level && Implied--
                //a=antacedent
                //clause = clause+antacedent
        let mut counter = 0;
        loop {
            counter+=1;
            if counter > 200{  //inperformant
                //println!("skipped");
                return Some((clause,level));
            }
            let mut stackElem = None;
            for l in &clause.literals {
                stackElem = self.findInStack(l.value(), level);
                if !stackElem.is_none(){
                    break;
                }
            }
            if let Some(elem) = stackElem {
                clause = clause.resolute(&elem);
            } else {
                break;
            }
        }
        
        let returnLevel = self.secondHighestLevel(&clause);
        return Some((clause, returnLevel));
        
    }
    
    
    fn secondHighestLevel(&self, clause: &TwoPointerClause) -> isize{
        let mut highest = -1;
        let mut secondHighest = -1;
        for lit in &clause.literals {
            let level = lit.value() as isize;
            if (level > highest){
                secondHighest = highest;
                highest = level;
            } else {
                secondHighest = cmp::max(secondHighest, level);
            }
        }
        if (highest != secondHighest) {
            return secondHighest+1;
        } else{
            return secondHighest;
        }
    }
    
    
    /// finds the decision level of an literal in the stack
    fn getLevel(&self, literalValue:usize) -> isize {
        for i in (0..self.stack.len()).rev() {
            match(self.stack[i]){
                StackElem::Implied(ref lit, level, _) => {
                    if lit.value() == literalValue {
                        return level;
                    }
                }
                StackElem::Chosen(ref lit, level) => {
                    if lit.value() == literalValue {
                        return level;
                    }
                }
            }
        }
        panic!("literal not found in stack");
        return 0;
    }
    
    
    /// finds the literal with the level in the Stack
    fn findInStack(&self, literalValue: usize, level: isize) -> Option<StackElem>{
        for i in (0..self.stack.len()).rev() {
            match(self.stack[i]){
                StackElem::Implied(ref lit, lvl, _) => {
                    if lvl != level {
                        return None;
                    }
                    if lit.value() == literalValue {
                        return Some(self.stack[i].clone());
                    }
                }
                StackElem::Chosen(_,_) => {
                    return None;
                }
            }
        }
        return None;
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
                Err(_) => {
                    return foundFormula;
                }
            }
        }
    }
    
    /// backtracks until the choice of the passed level
    fn backtrack(&mut self, level:isize, learntClause:TwoPointerClause){
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
                        //println!("Remove {}", literal.value());
                        self.formula.choose(literal.value(), None);  //unassign chosen with wrong level
                    }
                },
                StackElem::Implied(literal, _, _) => {
                    //println!("Remove {}", literal.value());
                    self.formula.choose(literal.value(), None);  //unassign implied
                }
            }
        }
    }

}

impl CdCl for CdClInstance{
    
    /// Constructor
    fn new(initialFormula: FormulaInstance, receiver:Option<Receiver<TwoPointerClause>>, senders:Vec<Sender<TwoPointerClause>>)->Self{
        return CdClInstance{formula: initialFormula, stack: vec!(), receiver: receiver, senders: senders};
    }

    /// checks wether the formula is satisfiable
    fn sat(&mut self)->bool{
        let mut random = rand::thread_rng();
        
        //first unitPropagation
    
        //println!("{:?}", self.formula.form_state());
        let mut level:isize = -1;
        if !self.unitPropagation(level).is_none(){
            //println!("UNSAT");
            return false;
        }
        
        
        while self.formula.hasUnassignedVars() {
            //unitPropagation already done
            let unassigned = self.getUnassignedVariable(&mut random);
            let chosen:SimpleLiteral;
            if random.gen() {
                chosen = SimpleLiteral::Positive(unassigned);  //TODO: wieder positive machen
                self.formula.choose(chosen.value(), Some(true));
            } else {
                chosen = SimpleLiteral::Negative(unassigned);
                self.formula.choose(chosen.value(), Some(false));
            }
            level += 1;
            self.stack.push(StackElem::Chosen(chosen, level));
            //println!("{:?}", level);
    
    
    
            self.checkReceiverForNewClauses();
            let mut conflict = self.unitPropagation(level);
            while !conflict.is_none() {  //backtracking (some failure)
                let result = self.conflictAnalysis(conflict.unwrap(), level);
                if result.is_none() {
                    return false;
                }
                let (newClause, backtrackLevel) = result.unwrap();
                level = backtrackLevel-1;
                self.foundNewClause(&newClause);
    
                /*for i in (0..self.stack.len()) {
                    println!("  {:?}", self.stack[i]);
                }
                println!("----------------------------------------------\nBacktrack-Level: {:?}\n----------------------------------------------", backtrackLevel);
                */self.backtrack(backtrackLevel, newClause);
                /*for i in (0..self.stack.len()) {
                    println!("  {:?}", self.stack[i]);
                }
                println!("----------------------------------------------\nChoosing...\n----------------------------------------------");
                */conflict = self.unitPropagation(level);
            }
        }
    
        for i in (0..self.stack.len()) {
            println!("{:?}", self.stack[i]);
        }
        println!("{:?}", self.formula.form_state());

        return true;
    }

}
