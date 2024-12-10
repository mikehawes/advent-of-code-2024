use advent_of_code_2024::day01::Vectors;

fn main() {
    let vectors = Vectors::read_input("day01/input.txt").unwrap();
    println!("Part 1: {}", vectors.total_distance());
    println!("Part 2: {}", vectors.similarity());
}
