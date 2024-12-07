use crate::day7::equation::Operator::{Add, Concatenate, Multiply};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct Equation {
    numbers: Vec<usize>,
    equals: usize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn apply(&self, left: usize, right: usize) -> usize {
        match self {
            Add => left + right,
            Multiply => left * right,
            Concatenate => concat(left, right),
        }
    }
}

fn concat(left: usize, right: usize) -> usize {
    let mut str = left.to_string();
    str.push_str(right.to_string().as_str());
    usize::from_str(str.as_str()).unwrap()
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
    pub fn is_possible_add_multiply(&self) -> bool {
        self.possible_operator_combinations(vec![Add, Multiply])
            .any(|_| true)
    }
    pub fn answer(&self) -> usize {
        self.equals
    }
    fn possible_operator_combinations(
        &self,
        operators: Vec<Operator>,
    ) -> impl Iterator<Item = Vec<Operator>> + use<'_> {
        self.all_operator_combinations(operators)
            .filter(|c| self.is_combination_possible(c))
    }
    fn all_operator_combinations(
        &self,
        operators: Vec<Operator>,
    ) -> impl Iterator<Item = Vec<Operator>> {
        let num_operators = self.numbers.len() - 1;
        let num_combinations = operators.len().pow(num_operators as u32);
        (0..num_combinations).map(move |i| {
            let digits = get_digits_with_base(i, operators.len(), num_operators);
            digits
                .iter()
                .map(|digit| operators[*digit])
                .collect::<Vec<Operator>>()
        })
    }
    fn is_combination_possible(&self, operators: &[Operator]) -> bool {
        let mut acc = self.numbers[0];
        for (i, operator) in operators.iter().enumerate() {
            let right = self.numbers[i + 1];
            acc = operator.apply(acc, right);
        }
        acc == self.equals
    }
}

fn get_digits_with_base(num: usize, base: usize, pad: usize) -> Vec<usize> {
    let mut digits = vec![];
    let mut acc = num;
    while acc > 0 {
        digits.push(acc % base);
        acc /= base;
    }
    digits.resize(pad, 0);
    digits
}

#[cfg(test)]
mod tests {
    use crate::day7::equation::Operator::{Add, Concatenate, Multiply};
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

    #[test]
    fn can_find_possible_combinations_for_three_operators() {
        let equation = Equation::parse("8: 2 2 2 2");
        assert_eq!(print_possible_combinations(&equation), vec!["+++", "*++"]);
    }

    #[test]
    fn can_find_all_combinations_for_two_operators_with_concatenate() {
        let equation = Equation::parse("9: 3 3 3");
        assert_eq!(
            print_all_combinations_with_concatenate(&equation),
            vec!["++", "*+", "|+", "+*", "**", "|*", "+|", "*|", "||"]
        );
    }

    fn print_possible_combinations(equation: &Equation) -> Vec<String> {
        print_combinations(
            equation
                .possible_operator_combinations(vec![Add, Multiply])
                .collect(),
        )
    }

    fn print_all_combinations(equation: &Equation) -> Vec<String> {
        print_combinations(
            equation
                .all_operator_combinations(vec![Add, Multiply])
                .collect(),
        )
    }

    fn print_all_combinations_with_concatenate(equation: &Equation) -> Vec<String> {
        print_combinations(
            equation
                .all_operator_combinations(vec![Add, Multiply, Concatenate])
                .collect(),
        )
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
                Concatenate => '|',
            })
            .collect()
    }
}
