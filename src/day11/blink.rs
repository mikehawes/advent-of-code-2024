use crate::day11::digits::{count_digits, split_even_digits};

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

pub fn count_stones_with_blinks(blinks: usize, stones: Vec<usize>) -> usize {
    let mut count = 0;
    let mut remaining = stones.clone();
    count_stones_with_acc(blinks, &mut remaining, &mut count);
    count
}

fn count_stones_with_acc(blinks: usize, stones: &mut Vec<usize>, count: &mut usize) {
    if blinks == 0 {
        *count += stones.len();
        if *count % 10_000_000 == 0 {
            println!("Count: {count}")
        }
        return;
    }
    while let Some(stone) = stones.pop() {
        count_stones_with_acc(blinks - 1, &mut blink_stone(stone), count);
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
