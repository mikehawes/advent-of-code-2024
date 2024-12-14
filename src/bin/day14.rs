use advent_of_code_2024::day14::robots::Robots;
use advent_of_code_2024::input::{file_in_src, input_to_string};
use std::fs::File;
use std::io::Write;

fn main() {
    let input = input_to_string("day14/input.txt").unwrap();
    let robots = Robots::parse(input.as_str());
    let floor = [101, 103];
    let print_times = (0..101).map(|i| 76 + i * 103);
    println!("Part 1: {}", robots.safety_factor_after_seconds(100, floor));
    println!("Writing part 2");
    let mut file = File::create(file_in_src("day14/output.txt")).unwrap();
    for (time, picture) in robots.print_at_times(print_times, floor) {
        writeln!(file, "t={time}").unwrap();
        writeln!(file, "{picture}").unwrap();
        writeln!(file).unwrap();
    }
    println!("Part 2 in day14/output.txt");
}
