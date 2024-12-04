use crate::day4::find_cursor::FindCursor;
use crate::day4::search_cursor::SearchCursor;

mod find_cursor;
mod search_cursor;

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
        count_horizontal_matches(self, "XMAS")
    }
}

fn count_horizontal_matches(word_search: &WordSearch, find: &str) -> i32 {
    let mut search_cursor = SearchCursor::top_left(word_search);
    let mut find_cursor = FindCursor::start(find);
    let mut matches = 0;
    loop {
        loop {
            if search_cursor.char() == find_cursor.char() {
                find_cursor.advance();
                if find_cursor.is_finished() {
                    find_cursor.reset();
                    matches += 1;
                }
            } else {
                find_cursor.reset();
            }
            if !search_cursor.advance_x() {
                break;
            }
        }
        if !search_cursor.advance_y() {
            return matches;
        }
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
