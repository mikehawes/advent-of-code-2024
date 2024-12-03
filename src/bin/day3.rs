use advent_of_code_2024::day3::Multiplications;
use advent_of_code_2024::input::input_to_string;

fn main() {
    let input = input_to_string("day3/input.txt").unwrap();
    let multiplications = Multiplications::parse(input.as_str());
    println!("Part 1: {}", multiplications.sum());
}
