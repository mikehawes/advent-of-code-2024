use advent_of_code_2024::day09::disk_map::DiskMap;
use advent_of_code_2024::input::input_to_string;

fn main() {
    let input = input_to_string("day09/input.txt").unwrap();
    let map = DiskMap::parse(input.as_str());
    println!(
        "Part 1: {}",
        map.build_file_system().compact_splitting_files().checksum()
    );
    println!(
        "Part 2: {}",
        map.compact_fitting_into_spaces()
            .build_file_system()
            .checksum()
    );
}
