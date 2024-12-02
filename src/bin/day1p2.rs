use advent_of_code_2024::day1::Vectors;

fn main() {
    print!(
        "Result: {}",
        Vectors::read_input("day1/input.txt").unwrap().similarity()
    );
}
