use crate::day5::rule::PageOrderingRule;
use crate::day5::update::Update;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct PageOrderIndex {
    higher_pages: HashMap<i32, Vec<i32>>,
}

impl PageOrderIndex {
    pub fn parse_rules(string: &str) -> PageOrderIndex {
        Self::from_rules(PageOrderingRule::parse_to_vec(string))
    }
    pub fn from_rules(rules: Vec<PageOrderingRule>) -> PageOrderIndex {
        let mut higher_pages = HashMap::new();
        for rule in rules {
            insert_index(&mut higher_pages, rule.lower_page, rule.higher_page);
        }
        PageOrderIndex { higher_pages }
    }
    pub fn matches(&self, update: &Update) -> bool {
        update
            .pairs()
            .all(|(left, right)| self.pair_matches(left, right))
    }
    fn pair_matches(&self, left: i32, right: i32) -> bool {
        if let Some(higher) = self.higher_pages.get(&right) {
            higher
                .iter()
                .all(|h| *h != left && self.pair_matches(left, *h))
        } else {
            true
        }
    }
}

fn insert_index(map: &mut HashMap<i32, Vec<i32>>, key: i32, value: i32) {
    match map.entry(key) {
        Entry::Occupied(o) => o.into_mut(),
        Entry::Vacant(v) => v.insert(Vec::new()),
    }
    .push(value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day5::update::Update;

    #[test]
    fn can_build_index_from_rules() {
        let index = PageOrderIndex::parse_rules("1|2\n2|3\n3|4");
        assert_eq!(
            index.higher_pages,
            HashMap::from([(1, vec![2]), (2, vec![3]), (3, vec![4])])
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
