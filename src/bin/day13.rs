use advent_of_code_2024::day13::claw_machines::ClawMachines;
use advent_of_code_2024::input::input_to_string;

fn main() {
    let input = input_to_string("day12/input.txt").unwrap();
    let machines = ClawMachines::parse(input.as_str());
    println!("Part 1: {}", machines.sum_min_tokens());
}
