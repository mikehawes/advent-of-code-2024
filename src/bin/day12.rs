use advent_of_code_2024::day12::garden_map::GardenMap;
use advent_of_code_2024::input::input_to_string;

fn main() {
    let input = input_to_string("day12/input.txt").unwrap();
    let map = GardenMap::parse(input.as_str());
    println!("Part 1: {}", map.sum_fencing_price());
    println!("Part 2: {}", map.sum_fencing_price_bulk_discount())
}
