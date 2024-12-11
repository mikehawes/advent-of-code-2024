use crate::day11::blink::blink_stones;
use std::collections::HashMap;

pub struct BlinkCache {
    count_by_stone: HashMap<usize, Vec<usize>>,
    max_blinks: usize,
}

impl BlinkCache {
    pub fn for_blinks(max_blinks: usize) -> BlinkCache {
        BlinkCache {
            count_by_stone: HashMap::new(),
            max_blinks,
        }
    }
    pub fn counts_for_stone(&mut self, stone: usize) -> &Vec<usize> {
        let stones_before = self.unique_stones();
        self.count_by_stone.entry(stone).or_insert_with(|| {
            let index = index_blink_counts_for_stone(stone, self.max_blinks);
            let stones_after = stones_before + 1;
            if stones_after % 10 == 0 {
                println!("Cached {stones_after} unique stones")
            }
            index
        })
    }
    pub fn unique_stones(&self) -> usize {
        self.count_by_stone.len()
    }
}

fn index_blink_counts_for_stone(stone: usize, blinks: usize) -> Vec<usize> {
    let mut counts = vec![1];
    let mut stones = vec![stone];
    for _ in 0..blinks {
        stones = blink_stones(stones);
        counts.push(stones.len());
    }
    counts
}

#[cfg(test)]
mod tests {
    use crate::day11::blink_cache::BlinkCache;

    #[test]
    fn can_blink_5_times() {
        let mut cache = BlinkCache::for_blinks(5);
        assert_eq!(cache.counts_for_stone(0)[5], 4)
    }

    #[test]
    fn can_blink_5_times_cached() {
        let mut cache = BlinkCache::for_blinks(5);
        cache.counts_for_stone(0);
        assert_eq!(cache.counts_for_stone(0)[5], 4)
    }

    #[test]
    fn can_blink_25_times() {
        let mut cache = BlinkCache::for_blinks(25);
        assert_eq!(cache.counts_for_stone(0)[25], 19778)
    }
}
