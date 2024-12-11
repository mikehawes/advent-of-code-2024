use crate::day11::blink::blink_stone_times;
use std::collections::HashMap;

pub struct BlinkCache {
    result_by_stone: HashMap<usize, Vec<usize>>,
    blinks: usize,
}

impl BlinkCache {
    pub fn precompute(blinks: usize, max_stone: usize) -> BlinkCache {
        let result_by_stone: HashMap<usize, Vec<usize>> = (0..max_stone)
            .map(|stone| (stone, blink_stone_times(stone, blinks)))
            .collect();
        BlinkCache {
            result_by_stone,
            blinks,
        }
    }
    pub fn blinks(&self) -> usize {
        self.blinks
    }
    pub fn blink(&mut self, stones: &[usize]) -> Vec<usize> {
        stones
            .iter()
            .flat_map(|stone| self.blink_stone(*stone).clone())
            .collect()
    }
    pub fn blink_stone(&self, stone: usize) -> Vec<usize> {
        if let Some(result) = self.result_by_stone.get(&stone) {
            result.clone()
        } else {
            blink_stone_times(stone, self.blinks)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::blink_cache::BlinkCache;

    #[test]
    fn can_blink_5_times() {
        let cache = BlinkCache::precompute(5, 5_000);
        assert_eq!(cache.blink_stone(0), vec![4048, 1, 4048, 8096])
    }
}
