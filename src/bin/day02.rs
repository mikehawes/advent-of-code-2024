use advent_of_code_2024::day02::Reports;

fn main() {
    let reports = Reports::read_input("day02/input.txt").unwrap();
    println!("Part 1: {}", reports.count_safe());
    println!("Part 2: {}", reports.count_safe_with_tolerance());
}
