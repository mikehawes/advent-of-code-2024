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
        self.count(|report| report.is_safe())
    }

    pub fn count_safe_with_tolerance(&self) -> usize {
        self.count(|report| report.is_safe_with_tolerance())
    }

    fn count<P>(&self, predicate: P) -> usize
    where
        P: FnMut(&&Report) -> bool,
    {
        self.reports.iter().filter(predicate).count()
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

    fn is_safe_with_tolerance(&self) -> bool {
        if self.is_safe() {
            true
        } else {
            for i in 0..self.levels.len() {
                if self.without_index(i).is_safe() {
                    return true;
                }
            }
            false
        }
    }

    fn without_index(&self, index: usize) -> Report {
        let mut levels = Vec::with_capacity(self.levels.len() - 1);
        levels.extend_from_slice(&self.levels[0..index]);
        levels.extend_from_slice(&self.levels[index + 1..]);
        Report { levels }
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
            if !is_diff_safe(diff, found_diff) {
                return false;
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
        assert_eq!(parse_reports_as_vecs("1 2 3"), vec![vec![1, 2, 3]])
    }

    #[test]
    fn can_parse_lines() {
        assert_eq!(
            parse_reports_as_vecs("1 2 3\n4 5 6"),
            vec![vec![1, 2, 3], vec![4, 5, 6]]
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
        assert!(is_safe_with_tolerance("1 2 3 3 4"));
    }

    #[test]
    fn can_refuse_two_duplicates() {
        assert!(!is_safe_with_tolerance("1 1 2 2"));
    }

    #[test]
    fn can_allow_big_jump_by_removing_second_item() {
        assert!(is_safe_with_tolerance("1 5 3"));
    }

    #[test]
    fn can_allow_big_jump_by_removing_first_item() {
        assert!(is_safe_with_tolerance("1 5 6"));
    }

    #[test]
    fn can_refuse_big_jump_when_jump_too_big_without_either_item() {
        assert!(!is_safe_with_tolerance("1 2 6 7"));
    }

    #[test]
    fn can_refuse_second_big_jump() {
        assert!(!is_safe_with_tolerance("1 5 3 7"));
    }

    #[test]
    fn can_refuse_direction_change_after_big_jump() {
        assert!(!is_safe_with_tolerance("1 2 6 1"));
    }

    #[test]
    fn can_allow_one_direction_change() {
        assert!(is_safe_with_tolerance("1 2 1 3"));
    }

    #[test]
    fn can_allow_duplicate_at_start() {
        assert!(is_safe_with_tolerance("1 1 2 3"));
    }

    #[test]
    fn can_allow_duplicate_at_end() {
        assert!(is_safe_with_tolerance("1 2 3 3"));
    }

    fn parse_reports_as_vecs(string: &str) -> Vec<Vec<i32>> {
        Reports::parse(string)
            .reports
            .iter()
            .map(|report| report.levels.clone())
            .collect()
    }

    fn is_safe_with_tolerance(string: &str) -> bool {
        Report::parse(string).is_safe_with_tolerance()
    }
}
