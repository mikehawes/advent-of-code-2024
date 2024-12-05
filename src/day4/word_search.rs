use crate::day4::find_cursor::FindCursor;
use crate::day4::lines::{generate_lines, Point};
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
pub struct WordSearch {
    tiles: Vec<Vec<char>>,
    width: usize,
}

impl WordSearch {
    pub fn parse(input: &str) -> WordSearch {
        let tiles: Vec<Vec<char>> = input.lines().map(read_chars).collect();
        let width = tiles.first().map(Vec::len).unwrap_or(0);
        WordSearch { tiles, width }
    }

    pub fn count_xmas(&self) -> i32 {
        let (matches, _) = self.count_xmas_find_points();
        matches
    }

    fn count_xmas_find_points(&self) -> (i32, HashSet<Point>) {
        let mut matches = 0;
        let mut find_cursor = FindCursor::start("XMAS");
        let mut points = HashSet::new();
        let mut current_points = HashSet::new();
        for line in generate_lines(self.width, self.tiles.len()) {
            for point in &line {
                if find_cursor.check_match_advance(self.char(point)) {
                    current_points.insert(*point);
                    if find_cursor.reset_if_finished() {
                        matches += 1;
                        current_points.iter().for_each(|p| {
                            points.insert(*p);
                        });
                        current_points.clear();
                    }
                } else {
                    current_points.clear();
                }
            }
            current_points.clear();
            find_cursor.reset();
            for point in line.iter().rev() {
                if find_cursor.check_match_advance(self.char(point)) {
                    current_points.insert(*point);
                    if find_cursor.reset_if_finished() {
                        matches += 1;
                        current_points.iter().for_each(|p| {
                            points.insert(*p);
                        });
                        current_points.clear();
                    }
                } else {
                    current_points.clear();
                }
            }
            find_cursor.reset();
        }
        (matches, points)
    }

    fn char(&self, point: &Point) -> char {
        let (x, y) = *point;
        self.tiles[y][x]
    }
}

fn read_chars(line: &str) -> Vec<char> {
    line.chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::input_to_string;
    use insta::assert_snapshot;
    use std::fmt::Write;

    #[test]
    fn can_find_no_xmas() {
        let word_search = WordSearch::parse("ABCD");
        assert_eq!(word_search.count_xmas(), 0)
    }

    #[test]
    fn can_find_xmas_one_line() {
        let word_search = WordSearch::parse("XMAS");
        assert_eq!(word_search.count_xmas(), 1)
    }

    #[test]
    fn can_find_xmas_two_lines() {
        let word_search = WordSearch::parse("XMAS\nXMAS");
        assert_eq!(word_search.count_xmas(), 2)
    }

    #[test]
    fn can_find_xmas_one_of_two_lines() {
        let word_search = WordSearch::parse("XMAS\nXMEN");
        assert_eq!(word_search.count_xmas(), 1)
    }

    #[test]
    fn can_find_xmas_over_lines() {
        let word_search = WordSearch::parse("X\nM\nA\nS");
        assert_eq!(word_search.count_xmas(), 1)
    }

    #[test]
    fn can_find_xmas_after_x() {
        let word_search = WordSearch::parse("XXMAS");
        assert_eq!(word_search.count_xmas(), 1)
    }

    #[test]
    fn can_find_xmas_in_example() {
        let example = input_to_string("day4/example.txt").unwrap();
        let word_search = WordSearch::parse(example.as_str());
        let (matches, points) = word_search.count_xmas_find_points();
        assert_eq!(matches, 18);
        assert_snapshot!(print_relevant_points(&word_search, &points))
    }

    #[test]
    fn can_find_xmas_diagonal() {
        let word_search = WordSearch::parse(
            "X___\n\
                   _M__\n\
                   __A_\n\
                   ___S",
        );
        assert_eq!(word_search.count_xmas(), 1)
    }

    #[test]
    fn can_parse_tiles() {
        let input = "\
            ABC\n\
            DEF\n\
            GEH";
        assert_eq!(
            WordSearch::parse(input),
            WordSearch {
                tiles: vec![
                    vec!['A', 'B', 'C'],
                    vec!['D', 'E', 'F'],
                    vec!['G', 'E', 'H']
                ],
                width: 3
            }
        );
    }
    fn print_relevant_points(word_search: &WordSearch, points: &HashSet<Point>) -> String {
        let mut str: String = "".to_owned();
        write_relevant_points(word_search, points, &mut str).unwrap();
        str
    }
    fn write_relevant_points<W: Write>(
        word_search: &WordSearch,
        points: &HashSet<Point>,
        out: &mut W,
    ) -> std::fmt::Result {
        for y in 0..word_search.tiles.len() {
            for x in 0..word_search.width {
                if points.contains(&(x, y)) {
                    out.write_char(word_search.char(&(x, y)))?;
                } else {
                    out.write_char('.')?;
                }
            }
            out.write_char('\n')?;
        }
        Ok(())
    }
}
