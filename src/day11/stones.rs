use crate::day11::blink::count_stones_with_blinks;
use crate::day11::blink_cache::BlinkCache;
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
        let mut cache = BlinkCache::for_blinks(25);
        self.count_stones_after_blinks_with_cache(times, &mut cache)
    }
    pub fn count_stones_after_blinks_with_cache(
        &self,
        times: usize,
        cache: &mut BlinkCache,
    ) -> usize {
        count_stones_with_blinks(times, self.stones.clone(), cache)
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::blink_cache::BlinkCache;
    use crate::day11::stones::Stones;

    #[test]
    fn can_parse_stones() {
        let stones = Stones::parse("0 1 10 99 999");
        assert_eq!(print(&stones), "0 1 10 99 999")
    }

    #[test]
    fn can_count_stones_after_25_blinks_second_example() {
        let stones = Stones::parse("125 17");
        assert_eq!(count_stones_after_blinks(&stones, 25), 55312)
    }

    #[test]
    fn can_find_number_of_stones_sequence() {
        let stones = Stones::parse("125 17");
        let sequence: Vec<usize> = (0..=25)
            .map(|blinks| count_stones_after_blinks(&stones, blinks))
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
            .map(|blinks| count_stones_after_blinks(&stones, blinks))
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
            .map(|blinks| count_stones_after_blinks(&stones, blinks))
            .collect();
        assert_eq!(
            sequence,
            vec![
                1, 1, 1, 2, 4, 4, 7, 14, 16, 20, 39, 62, 81, 110, 200, 328, 418, 667, 1059, 1546,
                2377, 3572, 5602, 8268, 12343, 19778, 29165, 43726, 67724, 102131, 156451
            ]
        )
    }

    fn count_stones_after_blinks(stones: &Stones, blinks: usize) -> usize {
        let mut cache = BlinkCache::for_blinks(10);
        stones.count_stones_after_blinks_with_cache(blinks, &mut cache)
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
