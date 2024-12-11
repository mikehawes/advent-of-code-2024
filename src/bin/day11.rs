use advent_of_code_2024::day11::stones::Stones;
use advent_of_code_2024::input::input_to_string;

fn main() {
    let input = input_to_string("day11/input.txt").unwrap();
    let stones = Stones::parse(input.as_str());
    println!("Part 1: {}", stones.count_stones_after_blinks(25));
    println!("Part 2: {}", stones.count_stones_after_blinks(75));
}
