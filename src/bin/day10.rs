use advent_of_code_2024::day10::hiking_map::HikingMap;
use advent_of_code_2024::input::input_to_string;

fn main() {
    let input = input_to_string("day09/input.txt").unwrap();
    let map = HikingMap::parse(input.as_str());
    println!("Part 1: {}", map.sum_trailhead_scores());
}
