use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct Equation {
    numbers: Vec<usize>,
    equals: usize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Operator {
    Add,
    Multiply,
}

impl Equation {
    pub fn parse_to_vec(string: &str) -> Vec<Equation> {
        string.lines().map(Equation::parse).collect()
    }
    fn parse(string: &str) -> Equation {
        let parts: Vec<&str> = string.split_whitespace().collect();
        let numbers = parts
            .iter()
            .skip(1)
            .flat_map(|str| usize::from_str(str))
            .collect();
        let equals = parts
            .first()
            .map(|str| usize::from_str(str.trim_end_matches(":")).unwrap())
            .unwrap();
        Equation { numbers, equals }
    }
    fn all_operator_combinations(&self) -> Vec<Vec<Operator>> {
        let num_operators = self.numbers.len() - 1;
        let num_combinations = 2_usize.pow(num_operators as u32);
        let mut out: Vec<Vec<Operator>> = Vec::with_capacity(num_combinations);
        for i in 0..num_combinations {
            let mut vec: Vec<Operator> = Vec::with_capacity(num_operators);
            for j in 0..num_operators {
                vec.push(match nth_binary_digit(i, j) {
                    0 => Operator::Add,
                    _ => Operator::Multiply,
                });
            }
            out.push(vec);
        }
        out
    }
}

fn nth_binary_digit(num: usize, digit: usize) -> usize {
    let one_at_digit = 2_usize.pow(digit as u32);
    let value_at_digit = num & one_at_digit;
    value_at_digit >> digit
}

#[cfg(test)]
mod tests {
    use crate::day7::equation::Operator::{Add, Multiply};
    use crate::day7::equation::{Equation, Operator};

    #[test]
    fn can_parse_equations() {
        let string = "4: 2 2\n5: 2 1";
        let equations = Equation::parse_to_vec(string);
        assert_eq!(
            equations,
            vec! {
                Equation{
                    numbers: vec![2, 2],
                    equals: 4
                },
                Equation {
                    numbers: vec![2, 1],
                    equals: 5
                }
            }
        )
    }

    #[test]
    fn can_find_all_combinations_for_one_operator() {
        let equation = Equation::parse("4: 2 2");
        assert_eq!(print_all_combinations(&equation), vec!["+", "*"]);
    }

    #[test]
    fn can_find_all_combinations_for_two_operators() {
        let equation = Equation::parse("9: 3 3 3");
        assert_eq!(
            print_all_combinations(&equation),
            vec!["++", "*+", "+*", "**"]
        );
    }

    #[test]
    fn can_find_all_combinations_for_three_operators() {
        let equation = Equation::parse("8: 2 2 2 2");
        assert_eq!(
            print_all_combinations(&equation),
            vec!["+++", "*++", "+*+", "**+", "++*", "*+*", "+**", "***"]
        );
    }

    fn print_all_combinations(equation: &Equation) -> Vec<String> {
        print_combinations(equation.all_operator_combinations())
    }

    fn print_combinations(combinations: Vec<Vec<Operator>>) -> Vec<String> {
        combinations.iter().map(|c| print_combination(c)).collect()
    }

    fn print_combination(combination: &[Operator]) -> String {
        combination
            .iter()
            .map(|op| match op {
                Add => '+',
                Multiply => '*',
            })
            .collect()
    }
}
