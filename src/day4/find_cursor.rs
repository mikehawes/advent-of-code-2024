pub struct FindCursor {
    find: Vec<char>,
    find_index: usize,
}

impl FindCursor {
    pub fn start(string: &str) -> FindCursor {
        FindCursor {
            find: string.chars().collect(),
            find_index: 0,
        }
    }
    pub fn char(&self) -> Option<char> {
        self.find.get(self.find_index).copied()
    }
    pub fn check_match_advance(&mut self, check: char) -> bool {
        if let Some(c) = self.char() {
            if c == check {
                self.advance();
                if self.is_finished() {
                    self.reset();
                    return true;
                }
            } else {
                self.reset();
            }
        }
        false
    }
    pub fn advance(&mut self) {
        self.find_index += 1;
    }
    pub fn reset(&mut self) {
        self.find_index = 0;
    }
    pub fn is_finished(&self) -> bool {
        self.find_index == self.find.len()
    }
}

impl Iterator for FindCursor {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let char = self.char();
        if char.is_some() {
            self.advance();
        }
        char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_iterate_over_chars() {
        let letters: Vec<char> = FindCursor::start("XMAS").collect();
        assert_eq!(letters, vec!['X', 'M', 'A', 'S'])
    }

    #[test]
    fn can_reset() {
        let mut cursor = FindCursor::start("AB");
        let first = [cursor.next(), cursor.next(), cursor.next()];
        cursor.reset();
        let second = [cursor.next(), cursor.next(), cursor.next()];

        assert_eq!(first, [Some('A'), Some('B'), None]);
        assert_eq!(second, [Some('A'), Some('B'), None]);
    }
}
