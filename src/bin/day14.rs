use advent_of_code_2024::day14::robots::Robots;
use advent_of_code_2024::input::input_to_string;

fn main() {
    let input = input_to_string("day14/input.txt").unwrap();
    let robots = Robots::parse(input.as_str());
    let floor = [101, 103];
    println!("Part 1: {}", robots.safety_factor_after_seconds(100, floor));
}
