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
        self.equations
            .iter()
            .filter(|e| e.is_possible())
            .map(|e| e.answer())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::equations::Equations;
    use crate::input::input_to_string;

    #[test]
    fn can_sum_example_possible_answers() {
        let input = input_to_string("day7/example.txt").unwrap();
        let sum = Equations::parse(input.as_str()).sum_possible_answers();
        assert_eq!(sum, 3749)
    }
}
