#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(unused_parens)]
#![allow(unused_must_use)]
#![allow(unused_assignments)]

mod formula;
mod clause;
mod literal;
mod cdcl;
mod reader;

use formula::*;
use clause::*;
use cdcl::*;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::vec::Vec;
use std::thread::{spawn, JoinHandle};
use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let defaultFile = "samples/simple.cnf";
    let formula = if args.len() > 1 {reader::read(&args[1])} else { reader::read(defaultFile) };
    if startSolver(4, formula) {
        process::exit(1);
    } else {
        process::exit(0);
    }
}



fn startSolver(threadAmount: usize, formula: FormulaInstance) -> bool{
    
    let mut solvers: Vec<CdClInstance> = Vec::new();
    let mut senders: Vec<Sender<TwoPointerClause>> = Vec::new();
    let mut receivers: Vec<Receiver<TwoPointerClause>> = Vec::new();
    let mut threads: Vec<JoinHandle<_>> = Vec::new();
    
    for _ in 0..threadAmount {
        let (sender, receiver) = channel();  //create channels
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
    
    
    for _ in 0..threadAmount {
        let mut solver = solvers.pop().unwrap();
        threads.push(spawn(move || {
            let res = solver.sat();
            return (res, solver.formula.assignments);
        }));
    }
    
    for _ in 0..threadAmount {
        if let Ok((Some(result), assignments)) = threads.pop().unwrap().join() {
            if result {
                print!("SAT - Assignments: [");
                for ass in assignments{
                    print!("{:}", if ass.unwrap() {"1"} else {"0"});
                }
                println!("]");
                return true;
            } else {
                println!("UNSAT");
                return false;
            }
        }
    }
    panic!("All threads returned None!");
    return false;
}
