use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct Equation {
    numbers: Vec<usize>,
    equals: usize,
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
}

#[cfg(test)]
mod tests {
    use crate::day7::equation::Equation;

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
}
