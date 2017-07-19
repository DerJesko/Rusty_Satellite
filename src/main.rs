#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(while_true)]
#![allow(unused_must_use)]

mod formula;
mod clause;
mod literal;
mod cdcl;

use formula::*;
use clause::*;
use literal::*;
use cdcl::*;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::vec::Vec;
use std::thread;
use std::thread::*;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use std::collections::HashSet;
use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
    //let formula = read("unsat.cnf");
    //let formula = read("simple.cnf");
    //let formula = read("backtrack.cnf");
    //let formula = read("uf20-01.cnf");
    let formula = read("uuf50-0100.cnf");
    //let formula = read("satPy5-29.cnf");
    //println!("Formula: {:?}", formula);
    let file = if args.len() > 1 {read(&args[1])} else { formula };
    startSolver(1, file);
    println!("I'm a Rustaman");
    /*
    let test_assignment = vec![
        Some(true),
        Some(false),
        Some(true),
        Some(false)
    ];
    let mut test_clause = clause::TwoPointerClause{
        literals: vec![literal::SimpleLiteral::Negative(0), literal::SimpleLiteral::Positive(1), literal::SimpleLiteral::Negative(2), literal::SimpleLiteral::Positive(3)],
        state: clause::ClauseState::Satisfied,
        pointer: (2,3)
    };
    test_clause.update_clause_state(&test_assignment);
    println!("{:?}", test_clause)
    */
}

fn read(file_name: &str) -> FormulaInstance {

    // open file we want to parse
    let f = File::open(file_name).expect("Failed to open file!");

    // skip comments
    let mut reader = BufReader::new(f);
    let mut problem_line = String::new();
    for line in (&mut reader).lines() {
        match line {
            Ok(s) => match s.chars().next() {
                Some('p') => {
                    problem_line = s;
                    break;
                },
                Some('c') => continue,
                _ => panic!("Wrong format!")
            },
            _ => panic!("Failed to read file!")
        }
    }

    // TODO: handle panics
    // get #variables and #clauses
    let vec = problem_line.split_whitespace().collect::<Vec<&str>>();
    let variables = vec[2].parse().expect("Converting #variables failed!");
    // let clauses = vec[3].parse().expect("Converting #clauses failed!");

    // read rest of file
    let mut rest = String::new();
    reader.read_to_string(&mut rest).expect("Failed to read the rest of the file!");

    // TODO: handle panics
    // parse clauses
    let split = rest.split_whitespace().collect::<Vec<&str>>();
    let mut vec = Vec::new();
    let mut set = HashSet::new();
    let mut literal = 0;
    for var in split {
        // TODO: why is there even a '%' in the file? have to check and fix that
        if var == "%" {
            break;
        }
        literal = var.parse().expect("Converting literal failed!");
        if literal == 0 {
            set.insert(Clause::new(vec));
            vec = Vec::new();
        } else if literal < 0 {
            let lit = SimpleLiteral::Negative(-literal as usize -1);
            if !vec.contains(&lit) {
                vec.push(lit);
            }
        } else {
            let lit = SimpleLiteral::Positive(literal as usize - 1);
            if !vec.contains(&lit) {
                vec.push(lit);
            }
        }
    }
    // return formula
    Formula::new(variables, set)

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
               println!("{:?}", solver.formula.assignments);
               println!("thread ({:}) said it is satisfiable!", i);
               process::exit(1);
            } else {
               println!("thread ({:}) said it is unsatisfiable!", i);
               process::exit(0);
            }
        }));
    }
    
    /*for t in &threads {
        t.join();
    }*/
    for i in 0..threadAmount {
        threads.pop().unwrap().join();
    }
}
