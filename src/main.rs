#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(while_true)]

mod formula;
mod clause;
mod literal;
mod cdcl;

use formula::*;
use clause::*;
use cdcl::*;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::vec::Vec;
use std::thread;
use std::thread::*;

fn main() {
    println!("I'm a Rustaman");
}


fn startSolver(threadAmount: usize, formula: FormulaInstance){
    
    let mut solvers: Vec<CdClInstance> = Vec::new();
    let mut senders: Vec<Sender<TwoPointerClause>> = Vec::new();
    let mut receivers: Vec<Receiver<TwoPointerClause>> = Vec::new();
    let mut threads: Vec<JoinHandle<_>> = Vec::new();
    
    for i in 0..threadAmount {
        let (sender, receiver) = mpsc::channel();  //create channels
        senders.push(sender);
        receivers.push(receiver);
    }
    
    for i in (0..threadAmount).rev() {
        
        let formulaClone = formula.clone();
        let currentReceiver:Receiver<TwoPointerClause> = receivers.pop().unwrap();
        let mut currentSenders:Vec<Sender<TwoPointerClause>> = Vec::new();
        
        for j in 0..threadAmount {
            if i != j {
                let senderClone = senders[j].clone();
                currentSenders.push(senderClone);
            }
        }
        
        solvers.push(CdClInstance::new(formulaClone, Some(currentReceiver), currentSenders));
    }
    
    
    for i in 0..threadAmount {
        let mut solver = solvers.pop().unwrap();
        threads.push(thread::spawn(move || {
           if solver.sat() {
               print!("thread ({:}) said it is satisfiable!", i);
           } else {
               print!("thread ({:}) said it is unsatisfiable!",i);
           }
        }));
    }
}
