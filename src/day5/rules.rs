use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct PageOrderingRule {
    lower_page: i32,
    higher_page: i32,
}

impl PageOrderingRule {
    pub fn parse_to_vec(string: &str) -> Vec<PageOrderingRule> {
        string.lines().map(PageOrderingRule::parse).collect()
    }
    pub fn parse(string: &str) -> PageOrderingRule {
        let (left, right) = string.split_once('|').unwrap();
        let lower_page = i32::from_str(left).unwrap();
        let higher_page = i32::from_str(right).unwrap();
        PageOrderingRule {
            lower_page,
            higher_page,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_rules() {
        assert_eq!(
            parse_to_tuples("123|456\n78|90\n"),
            vec![(123, 456), (78, 90)]
        )
    }

    fn parse_to_tuples(string: &str) -> Vec<(i32, i32)> {
        PageOrderingRule::parse_to_vec(string)
            .iter()
            .map(|rule| (rule.lower_page, rule.higher_page))
            .collect()
    }
}
