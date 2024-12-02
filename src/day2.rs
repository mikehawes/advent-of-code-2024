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
            .flat_map(|(index, level)| Pair::from(&self.levels, index, *level));
        let mut found_diff: i32 = 0;
        let mut remaining_tolerance = tolerance;
        while let Some(pair) = pairs.next() {
            if let Some(diff) =
                pair.next_safe_diff(found_diff, &mut pairs, &mut remaining_tolerance)
            {
                if diff != 0 {
                    found_diff = diff;
                }
            } else {
                return false;
            }
        }
        true
    }
}

struct Pair {
    before: Option<i32>,
    left: i32,
    right: i32,
}

impl Pair {
    fn from(levels: &Vec<i32>, index: usize, left: i32) -> Option<Pair> {
        levels.get(index + 1).map(|right| Pair {
            before: match index {
                0 => None,
                i => Some(levels[i - 1]),
            },
            left,
            right: *right,
        })
    }

    fn next_safe_diff(
        &self,
        found_diff: i32,
        mut pairs: impl Iterator<Item = Pair>,
        remaining_tolerance: &mut bool,
    ) -> Option<i32> {
        let diff = self.diff();
        if is_diff_safe(diff, found_diff) {
            return Some(diff);
        }
        if !*remaining_tolerance {
            return None;
        }
        *remaining_tolerance = false;
        if let Some(next_pair) = pairs.next() {
            let next = next_pair.right;
            let skip_right_diff = self.skip_right_diff(next);
            if is_diff_safe(skip_right_diff, found_diff) {
                return Some(skip_right_diff);
            }
            if let Some(drop_first_diff) = self.drop_first_diff(next) {
                if is_diff_safe(drop_first_diff, found_diff) {
                    return Some(drop_first_diff);
                }
            }
            None
        } else {
            Some(0)
        }
    }

    fn diff(&self) -> i32 {
        self.right - self.left
    }

    fn drop_first_diff(&self, next: i32) -> Option<i32> {
        match self.before {
            Some(_) => None,
            None => Some(next - self.right),
        }
    }

    fn skip_right_diff(&self, next: i32) -> i32 {
        next - self.left
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
        assert_eq!(is_safe_with_tolerance("1 2 3 3 4"), true);
    }

    #[test]
    fn can_refuse_two_duplicates() {
        assert_eq!(is_safe_with_tolerance("1 1 2 2"), false);
    }

    #[test]
    fn can_allow_big_jump_by_removing_second_item() {
        assert_eq!(is_safe_with_tolerance("1 5 3"), true);
    }

    #[test]
    fn can_allow_big_jump_by_removing_first_item() {
        assert_eq!(is_safe_with_tolerance("1 5 6"), true);
    }

    #[test]
    fn can_refuse_big_jump_when_jump_too_big_without_either_item() {
        assert_eq!(is_safe_with_tolerance("1 2 6 7"), false);
    }

    #[test]
    fn can_refuse_second_big_jump() {
        assert_eq!(is_safe_with_tolerance("1 5 3 7"), false);
    }

    #[test]
    fn can_refuse_direction_change_after_big_jump() {
        assert_eq!(is_safe_with_tolerance("1 2 6 1"), false);
    }

    #[test]
    fn can_allow_one_direction_change() {
        assert_eq!(is_safe_with_tolerance("1 2 1 3"), true);
    }

    #[test]
    fn can_allow_duplicate_at_start() {
        assert_eq!(is_safe_with_tolerance("1 1 2 3"), true);
    }

    #[test]
    fn can_allow_duplicate_at_end() {
        assert_eq!(is_safe_with_tolerance("1 2 3 3"), true);
    }

    fn parse_reports_as_vecs(string: &str) -> Vec<Vec<i32>> {
        Reports::parse(string)
            .reports
            .iter()
            .map(|report| report.levels.clone())
            .collect()
    }

    fn is_safe_with_tolerance(string: &str) -> bool {
        Report::parse(string).is_safe_with_tolerance_flag(true)
    }
}
