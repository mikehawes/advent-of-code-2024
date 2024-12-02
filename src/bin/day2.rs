use advent_of_code_2024::day2::Reports;

fn main() {
    let reports = Reports::read_input("day2/input.txt").unwrap();
    print!("Part 1: {}", reports.count_safe());
}
