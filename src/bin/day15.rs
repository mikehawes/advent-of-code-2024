use advent_of_code_2024::day15::robot_plan::RobotPlan;
use advent_of_code_2024::input::input_to_string;

fn main() {
    let input = input_to_string("day15/input.txt").unwrap();
    let plan = RobotPlan::parse(input.as_str());
    println!("Part 1: {}", plan.sum_gps_coordinates_at_end());
}
