use crate::day11::blink::blink_stones;
use std::collections::HashMap;
use std::rc::Rc;

pub struct BlinkCache {
    blinks_by_stone: HashMap<usize, Vec<Rc<Vec<usize>>>>,
    max_blinks: usize,
}

impl BlinkCache {
    pub fn for_blinks(max_blinks: usize) -> BlinkCache {
        BlinkCache {
            blinks_by_stone: HashMap::new(),
            max_blinks,
        }
    }
    pub fn count_for_stone(&mut self, stone: usize, blinks: usize) -> Option<usize> {
        self.stone_blinks(stone)
            .get(blinks)
            .map(|stones| stones.len())
    }
    pub fn stones_after_max_blinks(&mut self, stone: usize) -> Rc<Vec<usize>> {
        let blinks = self.max_blinks;
        self.stone_blinks(stone)[blinks].clone()
    }
    fn stone_blinks(&mut self, stone: usize) -> &mut Vec<Rc<Vec<usize>>> {
        let stones_before = self.unique_stones();
        self.blinks_by_stone.entry(stone).or_insert_with(|| {
            let index = index_blinks_for_stone(stone, self.max_blinks);
            let stones_after = stones_before + 1;
            if stones_after % 10 == 0 {
                println!("Cached {stones_after} unique stones")
            }
            index
        })
    }
    pub fn unique_stones(&self) -> usize {
        self.blinks_by_stone.len()
    }
    pub fn max_blinks(&self) -> usize {
        self.max_blinks
    }
}

fn index_blinks_for_stone(stone: usize, blinks: usize) -> Vec<Rc<Vec<usize>>> {
    let mut blinks_index = vec![Rc::new(vec![stone])];
    let mut stones = vec![stone];
    for _ in 0..blinks {
        stones = blink_stones(stones);
        blinks_index.push(Rc::new(stones.clone()));
    }
    blinks_index
}

#[cfg(test)]
mod tests {
    use crate::day11::blink_cache::BlinkCache;

    #[test]
    fn can_blink_5_times() {
        let mut cache = BlinkCache::for_blinks(5);
        assert_eq!(cache.count_for_stone(0, 5).unwrap(), 4)
    }

    #[test]
    fn can_blink_5_times_cached() {
        let mut cache = BlinkCache::for_blinks(5);
        cache.count_for_stone(0, 5);
        assert_eq!(cache.count_for_stone(0, 5).unwrap(), 4)
    }

    #[test]
    fn can_blink_25_times() {
        let mut cache = BlinkCache::for_blinks(25);
        assert_eq!(cache.count_for_stone(0, 25).unwrap(), 19778)
    }
}
