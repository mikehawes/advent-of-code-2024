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
        self.x += 1;
        self.x < self.search.width
    }
    pub fn next_x(&mut self) -> Option<char> {
        let char = self.char();
        self.advance_x();
        char
    }
    pub fn reset_x(&mut self) {
        self.x = 0;
    }
    pub fn reset_y(&mut self) {
        self.y = 0;
    }
    pub fn advance_y(&mut self) -> bool {
        self.y += 1;
        self.y < self.search.tiles.len()
    }
    pub fn advance_y_reset_x(&mut self) -> bool {
        self.reset_x();
        self.advance_y()
    }
    pub fn advance_x_reset_y(&mut self) -> bool {
        self.reset_y();
        self.advance_x()
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
        let output = [
            cursor.next_x(),
            cursor.next_x(),
            cursor.next_x(),
            cursor.next_x(),
        ];
        assert_eq!(output, [Some('A'), Some('B'), None, None])
    }

    #[test]
    fn can_reset_row() {
        let word_search = WordSearch::parse("AB");
        let mut cursor = SearchCursor::top_left(&word_search);
        let first = [cursor.next_x(), cursor.next_x(), cursor.next_x()];
        cursor.reset_x();
        let second = [cursor.next_x(), cursor.next_x(), cursor.next_x()];
        assert_eq!(first, [Some('A'), Some('B'), None]);
        assert_eq!(second, [Some('A'), Some('B'), None]);
    }

    #[test]
    fn can_read_rows() {
        let word_search = WordSearch::parse("A\nB");
        let mut cursor = SearchCursor::top_left(&word_search);
        let a = cursor.next_x();
        let a_to_b = cursor.advance_y_reset_x();
        let b = cursor.next_x();
        let b_to_end = cursor.advance_y_reset_x();
        assert_eq!(
            (a, a_to_b, b, b_to_end),
            (Some('A'), true, Some('B'), false)
        );
    }
}
