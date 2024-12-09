use advent_of_code_2024::day9::disk_map::DiskMap;
use advent_of_code_2024::input::input_to_string;

fn main() {
    let input = input_to_string("day9/input.txt").unwrap();
    let map = DiskMap::parse(input.as_str());
    let file_system = map.build_file_system();
    println!(
        "Part 1: {}",
        file_system.compact_splitting_files().checksum()
    );
}
