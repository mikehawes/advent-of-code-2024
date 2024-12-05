use std::str::FromStr;

pub struct Update {
    pages: Vec<i32>,
}

impl Update {
    pub fn parse_to_vec(string: &str) -> Vec<Update> {
        string.lines().map(Update::parse).collect()
    }

    pub fn parse(string: &str) -> Update {
        let pages = string
            .split(",")
            .map(|part| i32::from_str(part).unwrap())
            .collect();
        Update { pages }
    }

    pub fn pages(&self) -> impl Iterator<Item = &i32> {
        self.pages.iter()
    }

    pub fn pairs(&self) -> impl Iterator<Item = (i32, i32)> + use<'_> {
        self.pages
            .iter()
            .enumerate()
            .filter(|(i, _)| *i < self.pages.len() - 1)
            .map(|(i, page)| (*page, self.pages[i + 1]))
    }

    pub fn middle(&self) -> i32 {
        self.pages[self.pages.len() / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_updates() {
        assert_eq!(parse_to_vecs("1,2\n3,4"), vec![vec![1, 2], vec![3, 4]])
    }

    #[test]
    fn can_find_middle_page_when_odd() {
        let update = Update::parse("1,2,3");
        assert_eq!(update.middle(), 2);
    }

    #[test]
    fn can_find_pairs() {
        let pairs: Vec<(i32, i32)> = Update::parse("1,2,3").pairs().collect();
        assert_eq!(pairs, vec![(1, 2), (2, 3)])
    }

    fn parse_to_vecs(string: &str) -> Vec<Vec<i32>> {
        Update::parse_to_vec(string)
            .iter()
            .map(|update| update.pages.clone())
            .collect()
    }
}