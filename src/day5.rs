use crate::day5::rules::PageOrderingRule;

mod rules;

pub struct PageOrdering {
    rules: Vec<PageOrderingRule>,
}

impl PageOrdering {
    pub fn parse(string: &str) -> PageOrdering {
        PageOrdering {
            rules: PageOrderingRule::parse_to_vec(string),
        }
    }
    pub fn sum_correct_middle_pages(&self) -> i32 {
        todo!()
    }
}
