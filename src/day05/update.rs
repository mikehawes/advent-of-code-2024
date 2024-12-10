use crate::day05::rules_index::RulesIndex;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
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

    pub fn middle(&self) -> i32 {
        self.pages[self.pages.len() / 2]
    }

    pub fn clone(&self) -> Update {
        Update {
            pages: self.pages.clone(),
        }
    }

    pub fn sort(&self, rules_index: &RulesIndex) -> Update {
        let mut sort = self.clone();
        sort.pages
            .sort_by(|left, right| rules_index.compare(*left, *right));
        sort
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
    fn can_sort_by_rules() {
        let update = Update::parse("1,2,3");
        let rules = RulesIndex::parse_rules("1|2\n3|2\n1|3");
        assert_eq!(update.sort(&rules), Update::parse("1,3,2"));
    }

    fn parse_to_vecs(string: &str) -> Vec<Vec<i32>> {
        Update::parse_to_vec(string)
            .iter()
            .map(|update| update.pages.clone())
            .collect()
    }
}
