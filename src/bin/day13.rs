use advent_of_code_2024::day13::claw_machines::ClawMachines;
use advent_of_code_2024::input::input_to_string;

fn main() {
    let input = input_to_string("day13/input.txt").unwrap();
    let machines = ClawMachines::parse(input.as_str());
    println!("Part 1: {}", machines.sum_min_tokens());
    println!("Part 2: {}", machines.sum_min_tokens_with_unit_conversion());
}
