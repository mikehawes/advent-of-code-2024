use crate::day07::equation::Equation;

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
            .filter(|e| e.is_possible_add_multiply())
            .map(|e| e.answer())
            .sum()
    }
    pub fn sum_possible_answers_with_concat(&self) -> usize {
        self.equations
            .iter()
            .filter(|e| e.is_possible_add_multiply_concatenate())
            .map(|e| e.answer())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::day07::equations::Equations;
    use crate::input::input_to_string;

    #[test]
    fn can_sum_example_possible_answers() {
        let input = input_to_string("day07/example.txt").unwrap();
        let sum = Equations::parse(input.as_str()).sum_possible_answers();
        assert_eq!(sum, 3749)
    }

    #[test]
    fn can_sum_example_possible_answers_with_concat() {
        let input = input_to_string("day07/example.txt").unwrap();
        let sum = Equations::parse(input.as_str()).sum_possible_answers_with_concat();
        assert_eq!(sum, 11387)
    }
}
