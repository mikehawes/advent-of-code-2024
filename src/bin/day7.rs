use advent_of_code_2024::day7::equations::Equations;
use advent_of_code_2024::input::input_to_string;

fn main() {
    let input = input_to_string("day7/input.txt").unwrap();
    let room = Equations::parse(input.as_str());
    println!("Part 1: {}", room.sum_possible_answers());
}
