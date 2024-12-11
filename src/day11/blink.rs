use crate::day11::blink_cache::BlinkCache;
use crate::day11::digits::{count_digits, split_even_digits};
use std::collections::HashSet;

pub fn blink_stone_times(stone: usize, times: usize) -> Vec<usize> {
    let mut stones = vec![stone];
    for _ in 0..times {
        stones = stones
            .iter()
            .flat_map(|stone| blink_stone(*stone))
            .collect();
    }
    stones
}

pub fn count_stones_with_blinks(blinks: usize, stones: Vec<usize>, cache: &BlinkCache) -> usize {
    let mut count = 0;
    let mut remaining = stones.clone();
    let mut unique_stones = HashSet::new();
    count_stones_with_acc(
        blinks,
        &cache,
        &mut remaining,
        &mut count,
        &mut unique_stones,
    );
    count
}

fn count_stones_with_acc(
    blinks: usize,
    cache: &BlinkCache,
    stones: &mut Vec<usize>,
    count: &mut usize,
    unique_stones: &mut HashSet<usize>,
) {
    if blinks == 0 {
        let before = *count;
        let after = *count + stones.len();
        *count = after;
        if before / 10_000_000 != after / 10_000_000 {
            let num_unique = unique_stones.len();
            println!("Count {count}, num unique {num_unique}");
        }
        return;
    }
    while let Some(stone) = stones.pop() {
        unique_stones.insert(stone);
        if blinks >= cache.blinks() {
            count_stones_with_acc(
                blinks - cache.blinks(),
                cache,
                &mut cache.blink_stone(stone),
                count,
                unique_stones,
            );
        } else {
            count_stones_with_acc(
                blinks - 1,
                cache,
                &mut blink_stone(stone),
                count,
                unique_stones,
            );
        }
    }
}

pub fn blink_stone(stone: usize) -> Vec<usize> {
    let digits = count_digits(stone);
    match stone {
        0 => vec![1],
        even if digits % 2 == 0 => split_even_digits(even, digits),
        other => vec![other * 2024],
    }
}
