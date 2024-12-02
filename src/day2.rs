use crate::input::input_to_string;
use std::io;
use std::path::Path;

#[derive(Debug, Eq, PartialEq)]
pub struct Reports {
    reports: Vec<Report>,
}

impl Reports {
    pub fn read_input<P: AsRef<Path>>(path: P) -> io::Result<Reports> {
        Ok(Reports::parse(&input_to_string(path)?))
    }

    fn parse(string: &str) -> Reports {
        Reports {
            reports: string.lines().map(Report::parse).collect(),
        }
    }

    pub fn count_safe(&self) -> usize {
        self.reports
            .iter()
            .filter(|report| report.is_safe())
            .count()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn parse(string: &str) -> Report {
        Report {
            levels: string
                .split_whitespace()
                .map(|level| level.parse().unwrap())
                .collect(),
        }
    }

    fn is_safe(&self) -> bool {
        let pairs = self
            .levels
            .iter()
            .enumerate()
            .flat_map(|(index, level)| self.levels.get(index + 1).map(|next| (*level, *next)));
        let mut found_diff: i32 = 0;
        for (left, right) in pairs {
            let diff = right - left;
            if !(1..=3).contains(&diff.abs()) {
                return false;
            }
            if found_diff != 0 && found_diff.is_positive() != diff.is_positive() {
                return false;
            } else if diff != 0 {
                found_diff = diff;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_line() {
        assert_eq!(
            Reports::parse("1 2 3").reports,
            vec![Report {
                levels: vec![1, 2, 3]
            }]
        )
    }

    #[test]
    fn can_parse_lines() {
        assert_eq!(
            Reports::parse("1 2 3\n4 5 6").reports,
            vec![
                Report {
                    levels: vec![1, 2, 3]
                },
                Report {
                    levels: vec![4, 5, 6]
                }
            ]
        )
    }

    #[test]
    fn can_count_safe_in_example() -> io::Result<()> {
        assert_eq!(Reports::read_input("day2/example.txt")?.count_safe(), 2);
        Ok(())
    }
}
