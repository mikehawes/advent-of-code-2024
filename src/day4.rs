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
        0
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
