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
        self.count_safe_with_tolerance_flag(false)
    }

    pub fn count_safe_with_tolerance(&self) -> usize {
        self.count_safe_with_tolerance_flag(true)
    }

    fn count_safe_with_tolerance_flag(&self, tolerance: bool) -> usize {
        self.reports
            .iter()
            .filter(|report| report.is_safe_with_tolerance_flag(tolerance))
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

    fn is_safe_with_tolerance_flag(&self, tolerance: bool) -> bool {
        let mut pairs = self
            .levels
            .iter()
            .enumerate()
            .flat_map(|(index, level)| self.levels.get(index + 1).map(|next| (*level, *next)));
        let mut found_diff: i32 = 0;
        let mut remaining_tolerance = tolerance;
        while let Some((left, right)) = pairs.next() {
            let diff = right - left;
            if !is_diff_safe(diff, found_diff) {
                if !remaining_tolerance {
                    return false;
                } else {
                    if let Some((_, next)) = pairs.next() {
                        let next_diff = next - left;
                        if !is_diff_safe(next_diff, found_diff) {
                            return false;
                        } else if next_diff != 0 {
                            found_diff = next_diff;
                        }
                    }
                    remaining_tolerance = false;
                }
            } else if diff != 0 {
                found_diff = diff;
            }
        }
        true
    }
}

fn is_diff_safe(diff: i32, found_diff: i32) -> bool {
    (1..=3).contains(&diff.abs())
        && (found_diff == 0 || found_diff.is_positive() == diff.is_positive())
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

    #[test]
    fn can_count_safe_with_tolerance_in_example() -> io::Result<()> {
        assert_eq!(
            Reports::read_input("day2/example.txt")?.count_safe_with_tolerance(),
            4
        );
        Ok(())
    }

    #[test]
    fn can_tolerate_one_duplicate() {
        assert_eq!(
            Report::parse("1 2 3 3 4").is_safe_with_tolerance_flag(true),
            true
        );
    }

    #[test]
    fn can_refuse_two_duplicates() {
        assert_eq!(
            Report::parse("1 1 2 2").is_safe_with_tolerance_flag(true),
            false
        );
    }

    #[test]
    fn can_refuse_big_jump_when_skipped_jump_is_still_too_big() {
        assert_eq!(
            Report::parse("1 5 6").is_safe_with_tolerance_flag(true),
            false
        );
    }
}
