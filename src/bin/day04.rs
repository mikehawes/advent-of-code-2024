use advent_of_code_2024::day04::word_search::WordSearch;
use advent_of_code_2024::input::input_to_string;

fn main() {
    let input = input_to_string("day04/input.txt").unwrap();
    let word_search = WordSearch::parse(input.as_str());
    println!("Part 1: {}", word_search.count_xmas());
    println!("Part 2: {}", word_search.count_x_mas());
}
