use crate::day5::rule::PageOrderingRule;
use crate::day5::update::Update;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

pub struct PageOrderIndex {
    lower_pages: HashMap<i32, HashSet<i32>>,
}

impl PageOrderIndex {
    pub fn parse_rules(string: &str) -> PageOrderIndex {
        Self::from_rules(PageOrderingRule::parse_to_vec(string))
    }
    pub fn from_rules(rules: Vec<PageOrderingRule>) -> PageOrderIndex {
        let mut lower_pages = HashMap::new();
        for rule in rules {
            insert_index(&mut lower_pages, rule.higher_page, rule.lower_page);
        }
        PageOrderIndex { lower_pages }
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
        let index = PageOrderIndex::parse_rules("1|2\n2|3\n3|4");
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
        let index = PageOrderIndex::parse_rules("1|2\n2|3\n3|4");
        assert_eq!(index.matches(&Update::parse("1,2,3")), true)
    }

    #[test]
    fn can_check_order_violates_rule() {
        let index = PageOrderIndex::parse_rules("1|2");
        assert_eq!(index.matches(&Update::parse("2,1")), false)
    }
}
