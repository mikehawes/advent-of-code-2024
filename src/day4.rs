use crate::day4::find_cursor::FindCursor;
use crate::day4::lines::{generate_lines, Point};

mod find_cursor;
mod lines;

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
        let mut matches = 0;
        let mut find_cursor = FindCursor::start("XMAS");
        for line in generate_lines(self.width, self.tiles.len()) {
            for point in &line {
                if find_cursor.check_match_advance(self.char(point)) {
                    matches += 1;
                }
            }
            find_cursor.reset();
            for point in line.iter().rev() {
                if find_cursor.check_match_advance(self.char(point)) {
                    matches += 1;
                }
            }
            find_cursor.reset();
        }
        matches
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
}
