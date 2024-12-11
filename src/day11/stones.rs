use std::str::FromStr;

pub struct Stones {
    stones: Vec<usize>,
}

impl Stones {
    pub fn parse(string: &str) -> Stones {
        let stones = string
            .split_whitespace()
            .map(|stone| usize::from_str(stone).unwrap())
            .collect();
        Stones { stones }
    }
    pub fn blink_times(&mut self, _: usize) -> &Stones {
        self
    }
    pub fn count_stones(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::stones::Stones;

    #[test]
    fn can_parse_stones() {
        let stones = Stones::parse("0 1 10 99 999");
        assert_eq!(print(&stones), "0 1 10 99 999")
    }

    fn print(stones: &Stones) -> String {
        stones
            .stones
            .iter()
            .map(|stone| stone.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
