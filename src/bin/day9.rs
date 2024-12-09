use advent_of_code_2024::day9::disk_map::DiskMap;
use advent_of_code_2024::input::input_to_string;

fn main() {
    let input = input_to_string("day9/input.txt").unwrap();
    let map = DiskMap::parse(input.as_str());
    println!(
        "Part 1: {}",
        map.file_system_checksum_after_compacting_last_first()
    );
}
