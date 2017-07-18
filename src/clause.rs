use literal::{Literal, SimpleLiteral};
use cdcl::{StackElem, CdClInstance};

pub trait Clause {
    fn new(literal_list: Vec<SimpleLiteral>) -> Self;
    fn update_clause_state(&mut self, assignments: &Vec<Option<bool>>);
    fn chooseUnit(&self, assignments: &mut Vec<Option<bool>>) -> SimpleLiteral;
    fn resolute(&mut self, &StackElem) -> TwoPointerClause;
}

impl Clause for TwoPointerClause {

    fn new(literal_list: Vec<SimpleLiteral>) -> TwoPointerClause {
        TwoPointerClause{
            pointer: (0,{if literal_list.len() > 1 { 1 } else { 0 }}),
            literals: literal_list,
            state: ClauseState::Open
        }
    }

    fn update_clause_state(&mut self, assignments: &Vec<Option<bool>>) {
        if let ClauseState::Satisfied = self.state {
            return
        }

        let (mut pointer_1, mut pointer_2) = self.pointer;
        if self.literals[pointer_1].is_satisfied(assignments) | self.literals[pointer_2].is_satisfied(assignments) {
            self.state = ClauseState::Satisfied;
            return;
        }

        let mut amount_of_undefined: u8 = 0;

        if let Some(_) = assignments[self.literals[pointer_1].value()] {
            let mut i = 0;
            while i < self.literals.len() {
                if i != pointer_2 {
                    match self.literals[i] {
                        SimpleLiteral::Positive(var) => {
                            match assignments[var] {
                                None => { break; }
                                Some(assigned_bool) => {
                                    if assigned_bool {
                                        self.state = ClauseState::Satisfied;
                                        pointer_1 = i;
                                        return;
                                    }
                                }
                            }
                        },
                        SimpleLiteral::Negative(var) => {
                            match assignments[var] {
                                None => { break; }
                                Some(assigned_bool) => {
                                    if !assigned_bool {
                                        self.state = ClauseState::Satisfied;
                                        pointer_1 = i;
                                        return;
                                    }
                                }
                            }
                        }
                    }
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
                if i != pointer_1 {
                    match self.literals[i] {
                        SimpleLiteral::Positive(var) => {
                            match assignments[var] {
                                None => { break; }
                                Some(assigned_bool) => {
                                    if assigned_bool {
                                        self.state = ClauseState::Satisfied;
                                        pointer_2 = i;
                                        return;
                                    }
                                }
                            }
                        },
                        SimpleLiteral::Negative(var) => {
                            match assignments[var] {
                                None => { break; }
                                Some(assigned_bool) => {
                                    if !assigned_bool {
                                        self.state = ClauseState::Satisfied;
                                        pointer_2 = i;
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
                i += 1;
            }
            if i != self.literals.len() {
                amount_of_undefined += 1;
                pointer_2 = i;
            }
        } else { amount_of_undefined += 1; }

        if amount_of_undefined == 2 {
            self.state = ClauseState::Open;
            return;
        } else if amount_of_undefined == 1 {
            if assignments[self.literals[pointer_1].value()] == None {
                self.state = ClauseState::Unit(pointer_1);
                return;
            } else {
                self.state = ClauseState::Unit(pointer_2);
                return;
            }
        } else {
            self.state = ClauseState::Unsatisfiable;
            return;
        }

        panic!();
    }

    fn chooseUnit(&self, assignments: &mut Vec<Option<bool>>) -> SimpleLiteral {
        if let ClauseState::Unit(literal_index) = self.state {
            match self.literals[literal_index] {
                SimpleLiteral::Positive(variable_index) => {
                    assignments[variable_index] = Some(true);
                    return SimpleLiteral::Positive(variable_index);
                }
                SimpleLiteral::Negative(variable_index) => {
                    assignments[variable_index] = Some(false);
                    return SimpleLiteral::Negative(variable_index);
                }
            }
        } else { panic!("You should not be here") }
    }

    fn resolute(&mut self, elem: &StackElem) -> TwoPointerClause {
        let mut clause: TwoPointerClause = CdClInstance::getAntecedent(elem).unwrap();

        let mut index = 0;
        match *elem {
            StackElem::Implied(_, x, _) => index = x,
            _ => panic!("Elem should not be chosen!")
        }

        let pos = SimpleLiteral::Positive(index);
        let neg = SimpleLiteral::Negative(index);

        for l in &self.literals {
            clause.literals.push(l.clone());
        }
    
        clause.literals.retain(|x| *x != pos);
        clause.literals.retain(|x| *x != neg);  //TODO: funktioniert das retain?

        clause

    }

}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TwoPointerClause {
    pub literals: Vec<SimpleLiteral>,
    pub state: ClauseState,
    pointer: (usize,usize)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ClauseState {
    Open,
    Unit(usize),
    Unsatisfiable,
    Satisfied
}
