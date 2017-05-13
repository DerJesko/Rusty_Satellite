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

impl CdCl for CdClInstance{
    fn new(initialFormula: FormulaInstance)->Self{
        return CdClInstance{formula: initialFormula, stack: vec![]};
    }
    fn sat()->bool{
        return true;
    }



}
