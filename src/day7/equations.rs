use crate::day7::equation::Equation;

pub struct Equations {
    equations: Vec<Equation>,
}

impl Equations {
    pub fn parse(string: &str) -> Equations {
        Equations {
            equations: Equation::parse_to_vec(string),
        }
    }
    pub fn sum_possible_answers(&self) -> usize {
        self.equations.len()
    }
}
