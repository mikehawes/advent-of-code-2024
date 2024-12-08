use advent_of_code_2024::day8::antenna_map::AntennaMap;
use advent_of_code_2024::input::input_to_string;

fn main() {
    let input = input_to_string("day8/input.txt").unwrap();
    let map = AntennaMap::parse(input.as_str());
    println!("Part 1: {}", map.count_unique_antinode_locations());
}
