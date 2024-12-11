use crate::day11::blink_cache::BlinkCache;
use crate::day11::digits::{count_digits, split_even_digits};

pub fn blink_stones(stones: Vec<usize>) -> Vec<usize> {
    stones
        .iter()
        .flat_map(|stone| blink_stone(*stone))
        .collect()
}

pub fn count_stones_with_blinks(
    blinks: usize,
    stones: Vec<usize>,
    cache: &mut BlinkCache,
) -> usize {
    let mut count = 0;
    let mut remaining = stones.clone();
    count_stones_with_acc(blinks, cache, &mut remaining, &mut count);
    count
}

fn count_stones_with_acc(
    blinks: usize,
    cache: &mut BlinkCache,
    stones: &mut Vec<usize>,
    count: &mut usize,
) {
    while let Some(stone) = stones.pop() {
        if let Some(stone_count) = cache.counts_for_stone(stone).get(blinks) {
            let before = *count;
            let after = before + stone_count;
            *count = after;
            if before / 1_000_000_000_000 != after / 1_000_000_000_000 {
                let num_unique = cache.unique_stones();
                println!("Count {count}, num unique cached {num_unique}");
            }
        } else {
            count_stones_with_acc(blinks - 1, cache, &mut blink_stone(stone), count);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_blink_once_first_example() {
        assert_eq!(
            print(&blink_times(1, &[0, 1, 10, 99, 999])),
            "1 2024 1 0 9 9 2021976"
        )
    }

    #[test]
    fn can_blink_second_example() {
        let stones = vec![125, 17];
        let blinks: Vec<String> = [0, 1, 2, 3, 4, 5, 6]
            .iter()
            .map(|blinks| blink_times(*blinks, &stones))
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
    fn can_blink_zero() {
        let stones = vec![0];
        let blinks: Vec<String> = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
            .iter()
            .map(|blinks| blink_times(*blinks, &stones))
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

    fn blink_times(times: usize, stones: &[usize]) -> Vec<usize> {
        stones
            .iter()
            .flat_map(|stone| blink_stone_times(*stone, times))
            .collect()
    }
    fn blink_stone_times(stone: usize, times: usize) -> Vec<usize> {
        let mut stones = vec![stone];
        for _ in 0..times {
            stones = stones
                .iter()
                .flat_map(|stone| blink_stone(*stone))
                .collect();
        }
        stones
    }
    fn print(stones: &[usize]) -> String {
        stones
            .iter()
            .map(|stone| stone.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
