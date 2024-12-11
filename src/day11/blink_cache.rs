use crate::day11::blink::blink_stone_times;
use std::collections::HashMap;

pub struct BlinkCache {
    result_by_stone: HashMap<usize, Vec<usize>>,
    times: usize,
}

impl BlinkCache {
    pub fn with_times(times: usize) -> BlinkCache {
        BlinkCache {
            result_by_stone: HashMap::new(),
            times,
        }
    }
    pub fn blink(&mut self, stones: &[usize]) -> Vec<usize> {
        stones
            .iter()
            .flat_map(|stone| self.blink_stone(*stone).clone())
            .collect()
    }
    fn blink_stone(&mut self, stone: usize) -> &Vec<usize> {
        self.result_by_stone
            .entry(stone)
            .or_insert_with(|| blink_stone_times(stone, self.times))
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::blink_cache::BlinkCache;

    #[test]
    fn can_blink_5_times() {
        let mut cache = BlinkCache::with_times(5);
        assert_eq!(cache.blink(&[0]), vec![4048, 1, 4048, 8096])
    }
}
