use crate::day5::rules_index::RulesIndex;
use crate::day5::update::Update;

pub struct PageOrdering {
    rules_index: RulesIndex,
    updates: Vec<Update>,
}

impl PageOrdering {
    pub fn parse(string: &str) -> PageOrdering {
        let (rules_str, updates_str) = string.split_once("\n\n").unwrap();
        PageOrdering {
            rules_index: RulesIndex::parse_rules(rules_str),
            updates: Update::parse_to_vec(updates_str),
        }
    }
    pub fn sum_correct_middle_pages(&self) -> i32 {
        self.updates
            .iter()
            .filter(|update| self.is_correct(update))
            .map(Update::middle)
            .sum()
    }
    pub fn sum_corrected_middle_pages(&self) -> i32 {
        self.updates
            .iter()
            .filter(|update| !self.is_correct(update))
            .map(|update| self.sort(update).middle())
            .sum()
    }
    fn is_correct(&self, update: &Update) -> bool {
        self.rules_index.matches(update)
    }
    fn sort(&self, update: &Update) -> Update {
        update.sort(&self.rules_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::input_to_string;

    #[test]
    fn can_sum_correct_middle_pages_for_example() {
        let example = input_to_string("day5/example.txt").unwrap();
        let ordering = PageOrdering::parse(example.as_str());
        assert_eq!(ordering.sum_correct_middle_pages(), 143);
    }

    #[test]
    fn can_sum_corrected_middle_pages_for_example() {
        let example = input_to_string("day5/example.txt").unwrap();
        let ordering = PageOrdering::parse(example.as_str());
        assert_eq!(ordering.sum_corrected_middle_pages(), 123);
    }
}
