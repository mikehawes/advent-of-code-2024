use advent_of_code_2024::day1::Vectors;

fn main() {
    let vectors = Vectors::read_input("day1/input.txt").unwrap();
    println!("Part 1: {}", vectors.total_distance());
    println!("Part 2: {}", vectors.similarity());
}
