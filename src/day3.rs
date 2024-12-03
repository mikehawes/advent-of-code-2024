use regex::{Captures, Regex};
use std::str::FromStr;

pub struct Multiplications {
    operations: Vec<Multiplication>,
}

impl Multiplications {
    pub fn parse(string: &str) -> Multiplications {
        Multiplications {
            operations: parse_multiplications(string),
        }
    }
    pub fn sum(&self) -> i32 {
        self.operations.iter().map(Multiplication::result).sum()
    }
}

pub struct Operations {
    operations: Vec<Box<dyn Operation>>,
}

impl Operations {
    pub fn parse(string: &str) -> Operations {
        Operations {
            operations: parse_operations(string),
        }
    }
    pub fn run(&self) -> i32 {
        let mut context = Context {
            multiplication_enabled: true,
            total: 0,
        };
        self.operations.iter().for_each(|op| op.apply(&mut context));
        context.total
    }
}

struct Context {
    multiplication_enabled: bool,
    total: i32,
}

trait Operation {
    fn apply(&self, context: &mut Context);
}

struct Multiplication {
    a: i32,
    b: i32,
}

impl Operation for Multiplication {
    fn apply(&self, context: &mut Context) {
        if context.multiplication_enabled {
            context.total += self.result();
        }
    }
}

impl Multiplication {
    fn result(&self) -> i32 {
        self.a * self.b
    }
}

struct Configure {
    multiplication_enabled: bool,
}

impl Operation for Configure {
    fn apply(&self, context: &mut Context) {
        context.multiplication_enabled = self.multiplication_enabled
    }
}

fn parse_multiplications(string: &str) -> Vec<Multiplication> {
    Regex::new(r"mul\(([0-9]+),([0-9]+)\)")
        .unwrap()
        .captures_iter(string)
        .map(parse_multiplication)
        .collect()
}

fn parse_multiplication(captures: Captures) -> Multiplication {
    let a = i32::from_str(captures.get(1).unwrap().as_str()).unwrap();
    let b = i32::from_str(captures.get(2).unwrap().as_str()).unwrap();
    Multiplication { a, b }
}

fn parse_operations(string: &str) -> Vec<Box<dyn Operation>> {
    Regex::new(r"(do\(\))|(don't\(\))|mul\(([0-9]+),([0-9]+)\)")
        .unwrap()
        .captures_iter(string)
        .map(parse_operation)
        .collect()
}

fn parse_operation(captures: Captures) -> Box<dyn Operation> {
    if captures.get(1).is_some() {
        return Box::new(Configure {
            multiplication_enabled: true,
        });
    }
    if captures.get(2).is_some() {
        return Box::new(Configure {
            multiplication_enabled: false,
        });
    }
    let a = i32::from_str(captures.get(3).unwrap().as_str()).unwrap();
    let b = i32::from_str(captures.get(4).unwrap().as_str()).unwrap();
    Box::new(Multiplication { a, b })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_sum_one_multiplication() {
        assert_eq!(Multiplications::parse("mul(2,4)").sum(), 8)
    }

    #[test]
    fn can_sum_multiplications_in_example() {
        let example = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(Multiplications::parse(example).sum(), 161)
    }

    #[test]
    fn can_apply_operations_in_example() {
        let example = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(Operations::parse(example).run(), 48)
    }
}
