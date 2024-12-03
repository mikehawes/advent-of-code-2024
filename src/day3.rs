use regex::{Captures, Regex};
use std::str::FromStr;

pub struct Multiplications {
    operations: Vec<Multiplication>,
}

impl Multiplications {
    pub fn parse(string: &str) -> Multiplications {
        let operations: Vec<Multiplication> = Regex::new(r"mul\(([0-9]+),([0-9]+)\)")
            .unwrap()
            .captures_iter(string)
            .map(read_multiplication)
            .collect();
        Multiplications { operations }
    }
    pub fn sum(&self) -> i32 {
        self.operations.iter().map(Multiplication::result).sum()
    }
}

pub struct Multiplication {
    a: i32,
    b: i32,
}

impl Multiplication {
    pub fn result(&self) -> i32 {
        self.a * self.b
    }
}

fn read_multiplication(captures: Captures) -> Multiplication {
    let a = i32::from_str(captures.get(1).unwrap().as_str()).unwrap();
    let b = i32::from_str(captures.get(2).unwrap().as_str()).unwrap();
    Multiplication { a, b }
}

#[cfg(test)]
mod tests {
    use crate::day3::Multiplications;

    #[test]
    fn can_sum_one_multiplication() {
        assert_eq!(Multiplications::parse("mul(2,4)").sum(), 8)
    }

    #[test]
    fn can_sum_multiplications_in_example() {
        let example = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(Multiplications::parse(example).sum(), 161)
    }
}
