use crate::input::input_to_string;
use std::cmp::Ordering;
use std::io;
use std::path::Path;

pub fn find_total_distance_from_input<P: AsRef<Path>>(path: P) -> io::Result<i32> {
    let str = input_to_string(path)?;
    Ok(read_vectors(&str).total_distance())
}

pub fn find_similarity_from_input<P: AsRef<Path>>(path: P) -> io::Result<i32> {
    let str = input_to_string(path)?;
    Ok(read_vectors(&str).similarity())
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

    fn similarity(&self) -> i32 {
        let mut similarity = 0;
        let mut left_iter = self.left.iter();
        let mut right_iter = self.right.iter();
        let mut left_cursor = left_iter.next();
        let mut right_cursor = right_iter.next();
        while let Some(left) = left_cursor {
            let mut frequency = 0;
            while let Some(right) = right_cursor {
                match right.cmp(left) {
                    Ordering::Less => {}
                    Ordering::Equal => {
                        frequency += 1;
                    }
                    Ordering::Greater => break,
                }
                right_cursor = right_iter.next();
            }
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
    fn can_find_example_similarity() -> io::Result<()> {
        let result = find_similarity_from_input("day1/example.txt")?;
        assert_eq!(result, 31);
        Ok(())
    }

    #[test]
    fn can_find_similarity_when_last_right_is_in_left() -> io::Result<()> {
        assert_eq!(read_vectors("1   2\n2   2").similarity(), 4);
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
