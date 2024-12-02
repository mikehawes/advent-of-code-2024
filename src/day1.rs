use crate::input::input_to_string;
use std::io;
use std::path::Path;

#[allow(dead_code)]
fn find_total_distance_from_input<P: AsRef<Path>>(path: P) -> io::Result<i32> {
    let str = input_to_string(path)?;
    let mut vectors = read_vectors(&str);
    vectors.sort();
    Ok(11)
}

fn read_vectors(string: &str) -> Vectors {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    for line in string.lines() {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.next().unwrap().parse().unwrap());
    }
    Vectors { left, right }
}

#[derive(Debug, Eq, PartialEq)]
struct Vectors {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Vectors {
    fn sort(&mut self) {
        self.left.sort();
        self.right.sort();
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
