#[derive(Debug, Eq, PartialEq)]
pub struct WordSearch {
    tiles: Vec<Vec<char>>,
}

impl WordSearch {
    pub fn parse(input: &str) -> WordSearch {
        let tiles = input.lines().map(read_chars).collect();
        WordSearch { tiles }
    }
}

fn read_chars(line: &str) -> Vec<char> {
    line.chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
                ]
            }
        );
    }
}
