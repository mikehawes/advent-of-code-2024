use crate::input::input_to_string;
use std::io;
use std::path::Path;

pub fn find_total_distance_from_input<P: AsRef<Path>>(path: P) -> io::Result<i32> {
    let str = input_to_string(path)?;
    Ok(read_vectors(&str).total_distance())
}

fn read_vectors(string: &str) -> Vectors {
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

#[derive(Debug, Eq, PartialEq)]
struct Vectors {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Vectors {
    fn total_distance(&self) -> i32 {
        let mut total = 0;
        for (i, left) in self.left.iter().enumerate() {
            let right = self.right.get(i).unwrap();
            total += (right - left).abs();
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_example_distance() -> io::Result<()> {
        let result = find_total_distance_from_input("day1/example.txt")?;
        assert_eq!(result, 11);
        Ok(())
    }

    #[test]
    fn can_parse_line() {
        assert_eq!(
            read_vectors("1   2"),
            Vectors {
                left: vec![1],
                right: vec![2]
            }
        )
    }

    #[test]
    fn can_parse_lines() {
        assert_eq!(
            read_vectors("1   2\n3   4"),
            Vectors {
                left: vec![1, 3],
                right: vec![2, 4]
            }
        )
    }
}
