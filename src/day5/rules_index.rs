use crate::day5::page_ordering_rule::PageOrderingRule;
use crate::day5::update::Update;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

pub struct RulesIndex {
    lower_pages: HashMap<i32, HashSet<i32>>,
}

impl RulesIndex {
    pub fn parse_rules(string: &str) -> RulesIndex {
        Self::from_rules(PageOrderingRule::parse_to_vec(string))
    }
    pub fn from_rules(rules: Vec<PageOrderingRule>) -> RulesIndex {
        let mut lower_pages = HashMap::new();
        for rule in rules {
            insert_index(&mut lower_pages, rule.higher_page, rule.lower_page);
        }
        RulesIndex { lower_pages }
    }
    pub fn matches(&self, update: &Update) -> bool {
        update.pages().enumerate().all(|(i, page)| {
            if let Some(lower) = self.lower_pages.get(page) {
                update.pages().skip(i).all(|after| !lower.contains(after))
            } else {
                true
            }
        })
    }
    pub fn compare(&self, left: i32, right: i32) -> Ordering {
        if let Some(lowers) = self.lower_pages.get(&right) {
            if lowers.contains(&left) {
                return Less;
            }
        }
        if let Some(lowers) = self.lower_pages.get(&left) {
            if lowers.contains(&right) {
                return Greater;
            }
        }
        Equal
    }
}

fn insert_index(map: &mut HashMap<i32, HashSet<i32>>, key: i32, value: i32) {
    match map.entry(key) {
        Entry::Occupied(o) => o.into_mut(),
        Entry::Vacant(v) => v.insert(HashSet::new()),
    }
    .insert(value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day5::update::Update;

    #[test]
    fn can_build_index_from_rules() {
        let index = RulesIndex::parse_rules("1|2\n2|3\n3|4");
        assert_eq!(
            index.lower_pages,
            HashMap::from([
                (2, HashSet::from([1])),
                (3, HashSet::from([2])),
                (4, HashSet::from([3]))
            ])
        )
    }

    #[test]
    fn can_check_order_matches_rules() {
        let index = RulesIndex::parse_rules("1|2\n2|3\n3|4");
        assert_eq!(index.matches(&Update::parse("1,2,3")), true)
    }

    #[test]
    fn can_check_order_violates_rule() {
        let index = RulesIndex::parse_rules("1|2");
        assert_eq!(index.matches(&Update::parse("2,1")), false)
    }
}
