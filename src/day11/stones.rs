use crate::day11::digits::{count_digits, split_even_digits};
use std::str::FromStr;

#[derive(Clone)]
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
    pub fn blink_times(&self, times: usize) -> Stones {
        let mut stones = self.clone();
        for _ in 0..times {
            stones = stones.blink();
        }
        stones
    }
    pub fn count_stones(&self) -> usize {
        self.stones.len()
    }
    fn blink(&self) -> Stones {
        Stones {
            stones: self
                .stones
                .iter()
                .flat_map(|stone| blink_stone(*stone))
                .collect(),
        }
    }
}

fn blink_stone(stone: usize) -> Vec<usize> {
    let digits = count_digits(stone);
    match stone {
        0 => vec![1],
        even if digits % 2 == 0 => split_even_digits(even, digits),
        other => vec![other * 2024],
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

    #[test]
    fn can_blink_once_first_example() {
        let stones = Stones::parse("0 1 10 99 999");
        assert_eq!(print(&stones.blink_times(1)), "1 2024 1 0 9 9 2021976")
    }

    #[test]
    fn can_blink_second_example() {
        let stones = Stones::parse("125 17");
        let blinks: Vec<String> = [0, 1, 2]
            .iter()
            .map(|blinks| stones.blink_times(*blinks))
            .map(|stones| print(&stones))
            .collect();
        assert_eq!(blinks, vec!["125 17", "253000 1 7", "253 0 2024 14168"])
    }

    #[test]
    fn can_count_stones_after_25_blinks() {
        let stones = Stones::parse("0 1 10 99 999");
        assert_eq!(stones.blink_times(25).count_stones(), 55312)
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
