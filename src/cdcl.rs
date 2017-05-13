use formula::*;
use clause::*;
use literal::*;
use std::vec::Vec;


pub trait CdCl{
    fn new(initialFormula: FormulaInstance)->Self;
    fn sat()->bool;
}


pub struct CdClInstance{
    formula: FormulaInstance,
    stack: Vec<StackElem>
}

pub enum StackElem{
    Implied(SimpleLiteral),
    Chosen(SimpleLiteral)
}

impl CdClInstance{

    fn unitPropagation(&mut self){
        /*while true {
            match self.formula.sat_state() {
                FormulaState::Unit(clauseIndex) => self.formula.chooseUnit(clauseIndex),
                FormulaState::Conflict(clauseIndex) => return clauseIndex,
                FormulaState::Else => return 0
            }
        }*/
    }
}

impl CdCl for CdClInstance{
    fn new(initialFormula: FormulaInstance)->Self{
        return CdClInstance{formula: initialFormula, stack: vec![]};
    }
    fn sat()->bool{
        return true;
    }

}
