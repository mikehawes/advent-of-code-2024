use advent_of_code_2024::day1::Vectors;

fn main() {
    let vectors = Vectors::read_input("day1/input.txt").unwrap();
    print!("Part 1: {}", vectors.total_distance());
    print!("Part 2: {}", vectors.similarity());
}
