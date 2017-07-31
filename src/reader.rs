use formula::*;
use clause::*;
use literal::*;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use std::collections::HashSet;


pub fn read(file_name: &str) -> FormulaInstance {
    
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
    
    // get #variables and #clauses
    let vec = problem_line.split_whitespace().collect::<Vec<&str>>();
    let variables = vec[2].parse().expect("Converting #variables failed!");
    
    // read rest of file
    let mut rest = String::new();
    reader.read_to_string(&mut rest).expect("Failed to read the rest of the file!");
    
    // parse clauses
    let split = rest.split_whitespace().collect::<Vec<&str>>();
    let mut vec = Vec::new();
    let mut set = HashSet::new();
    let mut literal = 0;
    for var in split {
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