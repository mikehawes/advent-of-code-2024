use advent_of_code_2024::day6::lab_room::LabRoom;
use advent_of_code_2024::input::input_to_string;

fn main() {
    let input = input_to_string("day6/input.txt").unwrap();
    let room = LabRoom::parse(input.as_str());
    println!("Part 1: {}", room.count_visited_positions());
    println!("Part 2: {}", room.count_positions_to_obstruct());
}
