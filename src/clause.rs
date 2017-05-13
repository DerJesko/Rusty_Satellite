use literal::{Literal, SimpleLiteral};

pub trait Clause {
    fn new(Vec<SimpleLiteral>) -> Self;
    fn clause_state(&mut self, assignments: &Vec<Option<bool>>) -> ClauseState;
}

impl Clause for TwoPointerClause {

    fn new(literal_list: Vec<SimpleLiteral>) -> TwoPointerClause {
        TwoPointerClause{
            pointer: (0,{if literal_list.len() > 1 { 1 } else { 0 }}),
            literals: literal_list,
            state: ClauseState::Open
        }
    }

    fn clause_state(&mut self, assignments: &Vec<Option<bool>>) -> ClauseState {
        if let ClauseState::Satisfied = self.state {
            return ClauseState::Satisfied;
        }

        let (mut pointer_1, mut pointer_2) = self.pointer;
        if self.literals[pointer_1].is_satisfied(assignments) | self.literals[pointer_2].is_satisfied(assignments) {
            self.state = ClauseState::Satisfied;
            return ClauseState::Satisfied;
        }

        let mut amount_of_undefined: u8 = 0;

        if let Some(_) = assignments[self.literals[pointer_1].value()] {
            let mut i = 0;
            while i < self.literals.len() {
                if (i != pointer_2) & (assignments[self.literals[i].value()] == None) {
                    break;
                }
                i += 1;
            }
            if i != self.literals.len() {
                amount_of_undefined += 1;
                pointer_1 = i;
            }
        } else { amount_of_undefined += 1; }

        if let Some(_) = assignments[self.literals[pointer_2].value()] {
            let mut i = 0;
            while i < self.literals.len() {
                if (i != pointer_1) & (assignments[self.literals[i].value()] == None) {
                    break;
                }
                i += 1;
            }
            if i != self.literals.len() {
                amount_of_undefined += 1;
                pointer_2 = i;
            }
        } else { amount_of_undefined += 1; }

        if amount_of_undefined == 2 {
            return ClauseState::Open;
        } else if amount_of_undefined == 1 {
            if assignments[self.literals[pointer_1].value()] == None {
                return ClauseState::Unit(pointer_1);
            } else {
                return ClauseState::Unit(pointer_2);
            }
        } else { return ClauseState::Filled }

        panic!();
    }
}

#[derive(Debug)]
pub struct TwoPointerClause {
    literals: Vec<SimpleLiteral>,
    state: ClauseState,
    pointer: (usize,usize)
}

#[derive(Debug)]
pub enum ClauseState {
    Open,
    Unit(usize),
    Satisfied,
    Filled,
}
