use formula;
use clause;
use literal;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use std::collections::HashSet;


fn read(file_name: str) -> TwoPointerClause {

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
    let variables = vec[2];
    let clauses = vec[3];

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
        literal = var.parse().expect("Converting literal failed!");
        if literal == 0 {
            set.insert(Clause::new(vec));
            vec = Vec::new();
            set = HashSet::new();
        } else if literal < 0 {
            vec.push(SimpleLiteral::Negative(-literal));
        } else {
            vec.push(SimpleLiteral::Positive(literal));
        }
    }

    // return formula
    Formula::new(set)

}