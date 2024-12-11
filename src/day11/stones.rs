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
    pub fn count_stones_after_blinks(&self, times: usize) -> usize {
        self.blink_times(times).count_stones()
    }
    fn blink_times(&self, times: usize) -> Stones {
        let mut stones = self.clone();
        for _ in 0..times {
            stones = stones.blink();
        }
        stones
    }
    fn count_stones(&self) -> usize {
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
        let blinks: Vec<String> = [0, 1, 2, 3, 4, 5, 6]
            .iter()
            .map(|blinks| stones.blink_times(*blinks))
            .map(|stones| print(&stones))
            .collect();
        assert_eq!(
            blinks,
            vec![
                "125 17",
                "253000 1 7",
                "253 0 2024 14168",
                "512072 1 20 24 28676032",
                "512 72 2024 2 0 2 4 2867 6032",
                "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32",
                "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2"
            ]
        )
    }

    #[test]
    fn can_count_stones_after_25_blinks_second_example() {
        let stones = Stones::parse("125 17");
        assert_eq!(stones.blink_times(25).count_stones(), 55312)
    }

    #[test]
    fn can_find_number_of_stones_sequence() {
        let stones = Stones::parse("125 17");
        let sequence: Vec<usize> = (0..=25)
            .map(|blinks| stones.blink_times(blinks))
            .map(|stones| stones.count_stones())
            .collect();
        assert_eq!(
            sequence,
            vec![
                2, 3, 4, 5, 9, 13, 22, 31, 42, 68, 109, 170, 235, 342, 557, 853, 1298, 1951, 2869,
                4490, 6837, 10362, 15754, 23435, 36359, 55312
            ]
        )
    }

    #[test]
    fn can_find_number_of_stones_sequence_for_one_stone() {
        let stones = Stones::parse("125");
        let sequence: Vec<usize> = (0..=30)
            .map(|blinks| stones.blink_times(blinks))
            .map(|stones| stones.count_stones())
            .collect();
        assert_eq!(
            sequence,
            vec![
                1, 1, 2, 2, 3, 5, 7, 9, 15, 26, 38, 56, 82, 126, 189, 283, 467, 682, 978, 1575,
                2395, 3625, 5390, 8173, 12846, 19025, 28994, 44502, 66490, 102144, 154793
            ]
        )
    }

    #[test]
    fn can_find_number_of_stones_sequence_for_zero() {
        let stones = Stones::parse("0");
        let sequence: Vec<usize> = (0..=30)
            .map(|blinks| stones.blink_times(blinks))
            .map(|stones| stones.count_stones())
            .collect();
        assert_eq!(
            sequence,
            vec![
                1, 1, 1, 2, 4, 4, 7, 14, 16, 20, 39, 62, 81, 110, 200, 328, 418, 667, 1059, 1546,
                2377, 3572, 5602, 8268, 12343, 19778, 29165, 43726, 67724, 102131, 156451
            ]
        )
    }

    #[test]
    fn can_blink_zero() {
        let stones = Stones::parse("0");
        let blinks: Vec<String> = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
            .iter()
            .map(|blinks| stones.blink_times(*blinks))
            .map(|stones| print(&stones))
            .collect();
        assert_eq!(
            blinks,
            vec!["0",
                 "1",
                 "2024",
                 "20 24",
                 "2 0 2 4",
                 "4048 1 4048 8096",
                 "40 48 2024 40 48 80 96",
                 "4 0 4 8 20 24 4 0 4 8 8 0 9 6",
                 "8096 1 8096 16192 2 0 2 4 8096 1 8096 16192 16192 1 18216 12144",
                 "80 96 2024 80 96 32772608 4048 1 4048 8096 80 96 2024 80 96 32772608 32772608 2024 36869184 24579456",
                 "8 0 9 6 20 24 8 0 9 6 3277 2608 40 48 2024 40 48 80 96 8 0 9 6 20 24 8 0 9 6 3277 2608 3277 2608 20 24 3686 9184 2457 9456"]
        )
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
