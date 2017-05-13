use formula;
use clause;
use literal;


pub trait CdCl{
    fn new(initialFormula: formula::FormulaInstance)->Self;
    fn sat()->bool;
}


pub struct CdClInstance{
    formula: formula::FormulaInstance
}

impl CdCl for CdClInstance{
    fn new(initialFormula: formula::FormulaInstance)->Self{
        return CdClInstance{formula: initialFormula};
    }
    fn sat()->bool{
        return true;
    }
}
