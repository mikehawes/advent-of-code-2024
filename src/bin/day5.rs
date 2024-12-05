use advent_of_code_2024::day5::page_ordering::PageOrdering;
use advent_of_code_2024::input::input_to_string;

fn main() {
    let input = input_to_string("day5/input.txt").unwrap();
    let ordering = PageOrdering::parse(input.as_str());
    println!("Part 1: {}", ordering.sum_correct_middle_pages());
    println!("Part 2: {}", ordering.sum_corrected_middle_pages());
}
