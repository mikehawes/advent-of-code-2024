use crate::input::input_to_string;
use std::cmp::Ordering;
use std::iter::Peekable;
use std::path::Path;
use std::{io, slice};

#[derive(Debug, Eq, PartialEq)]
pub struct Vectors {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Vectors {
    pub fn read_input<P: AsRef<Path>>(path: P) -> io::Result<Vectors> {
        Ok(Vectors::parse(&input_to_string(path)?))
    }

    pub fn parse(string: &str) -> Vectors {
        let mut left: Vec<i32> = vec![];
        let mut right: Vec<i32> = vec![];
        for line in string.lines() {
            let mut parts = line.split_whitespace();
            left.push(parts.next().unwrap().parse().unwrap());
            right.push(parts.next().unwrap().parse().unwrap());
        }
        left.sort();
        right.sort();
        Vectors { left, right }
    }

    pub fn total_distance(&self) -> i32 {
        let mut total = 0;
        for (i, left) in self.left.iter().enumerate() {
            let right = self.right.get(i).unwrap();
            total += (right - left).abs();
        }
        total
    }

    pub fn similarity(&self) -> i32 {
        let mut similarity = 0;
        let mut left_iter = self.left.iter();
        let mut right_iter = self.right.iter().peekable();
        let mut left_cursor = left_iter.next();
        while let Some(left) = left_cursor {
            let frequency = scan_frequency(*left, &mut right_iter);
            loop {
                similarity += left * frequency;
                left_cursor = left_iter.next();
                if let Some(next) = left_cursor {
                    if next != left {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        similarity
    }
}

fn scan_frequency(left: i32, right_iter: &mut Peekable<slice::Iter<i32>>) -> i32 {
    let mut frequency = 0;
    while let Some(right) = right_iter.peek() {
        match left.cmp(*right) {
            Ordering::Greater => {}
            Ordering::Equal => {
                frequency += 1;
            }
            Ordering::Less => break,
        }
        right_iter.next();
    }
    frequency
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_example_distance() -> io::Result<()> {
        let result = Vectors::read_input("day01/example.txt")?.total_distance();
        assert_eq!(result, 11);
        Ok(())
    }

    #[test]
    fn can_find_example_similarity() -> io::Result<()> {
        let result = Vectors::read_input("day01/example.txt")?.similarity();
        assert_eq!(result, 31);
        Ok(())
    }

    #[test]
    fn can_find_similarity_when_last_right_is_in_left() -> io::Result<()> {
        assert_eq!(Vectors::parse("1   2\n2   2").similarity(), 4);
        Ok(())
    }

    #[test]
    fn can_parse_line() {
        assert_eq!(
            Vectors::parse("1   2"),
            Vectors {
                left: vec![1],
                right: vec![2]
            }
        )
    }

    #[test]
    fn can_parse_lines() {
        assert_eq!(
            Vectors::parse("1   2\n3   4"),
            Vectors {
                left: vec![1, 3],
                right: vec![2, 4]
            }
        )
    }
}
