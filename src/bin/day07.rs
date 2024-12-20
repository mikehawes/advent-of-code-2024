use advent_of_code_2024::day07::equations::Equations;
use advent_of_code_2024::input::input_to_string;

fn main() {
    let input = input_to_string("day07/input.txt").unwrap();
    let room = Equations::parse(input.as_str());
    println!("Part 1: {}", room.sum_possible_answers());
    println!("Part 2: {}", room.sum_possible_answers_with_concat());
}
