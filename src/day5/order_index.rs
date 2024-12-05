use crate::day5::rule::PageOrderingRule;
use crate::day5::update::Update;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct PageOrderIndex {
    lower_pages: HashMap<i32, Vec<i32>>,
    higher_pages: HashMap<i32, Vec<i32>>,
}

impl PageOrderIndex {
    pub fn parse_rules(string: &str) -> PageOrderIndex {
        Self::from_rules(PageOrderingRule::parse_to_vec(string))
    }
    pub fn from_rules(rules: Vec<PageOrderingRule>) -> PageOrderIndex {
        let mut lower_pages = HashMap::new();
        let mut higher_pages = HashMap::new();
        for rule in rules {
            insert_index(&mut lower_pages, rule.higher_page, rule.lower_page);
            insert_index(&mut higher_pages, rule.lower_page, rule.higher_page);
        }
        PageOrderIndex {
            lower_pages,
            higher_pages,
        }
    }
    pub fn matches(&self, update: &Update) -> bool {
        true
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
            index.lower_pages,
            HashMap::from([(2, vec![1]), (3, vec![2]), (4, vec![3])])
        );
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
}
