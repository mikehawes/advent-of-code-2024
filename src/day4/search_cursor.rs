use crate::day4::WordSearch;

pub struct SearchCursor<'a> {
    search: &'a WordSearch,
    x: usize,
    y: usize,
}

impl<'a> SearchCursor<'a> {
    pub fn top_left(search: &'a WordSearch) -> SearchCursor<'a> {
        SearchCursor { search, x: 0, y: 0 }
    }

    pub fn char(&self) -> Option<char> {
        self.search
            .tiles
            .get(self.y)
            .and_then(|line| line.get(self.x).copied())
    }
    pub fn advance_x(&mut self) -> bool {
        if self.x < self.search.width {
            self.x += 1;
            true
        } else {
            false
        }
    }
    pub fn next_x(&mut self) -> Option<char> {
        let char = self.char();
        self.advance_x();
        char
    }
    pub fn reset_x(&mut self) {
        self.x = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_top_left() {
        let word_search = WordSearch::parse("ABCD");
        let cursor = SearchCursor::top_left(&word_search);
        assert_eq!(cursor.char(), Some('A'))
    }

    #[test]
    fn can_read_row() {
        let word_search = WordSearch::parse("AB");
        let mut cursor = SearchCursor::top_left(&word_search);
        let output = vec![
            cursor.next_x(),
            cursor.next_x(),
            cursor.next_x(),
            cursor.next_x(),
        ];
        assert_eq!(output, vec![Some('A'), Some('B'), None, None])
    }

    #[test]
    fn can_reset_row() {
        let word_search = WordSearch::parse("AB");
        let mut cursor = SearchCursor::top_left(&word_search);
        let first = vec![cursor.next_x(), cursor.next_x(), cursor.next_x()];
        cursor.reset_x();
        let second = vec![cursor.next_x(), cursor.next_x(), cursor.next_x()];
        assert_eq!(first, vec![Some('A'), Some('B'), None]);
        assert_eq!(second, vec![Some('A'), Some('B'), None]);
    }
}
