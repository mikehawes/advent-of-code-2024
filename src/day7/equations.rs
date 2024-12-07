use crate::day7::equation::Equation;

pub struct Equations {
    equations: Vec<Equation>,
}

impl Equations {
    pub fn parse(_: &str) -> Equations {
        Equations { equations: vec![] }
    }
    pub fn sum_possible_answers(&self) -> usize {
        self.equations.len()
    }
}
