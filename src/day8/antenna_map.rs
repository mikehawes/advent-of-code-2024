use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct AntennaMap {
    width: usize,
    height: usize,
    positions_by_frequency: HashMap<char, Vec<Point>>,
}

pub(super) type Point = (usize, usize);

impl AntennaMap {
    pub fn parse(string: &str) -> AntennaMap {
        let mut width = 0;
        let mut height = 0;
        let mut positions_by_frequency = HashMap::new();
        for (y, line) in string.lines().enumerate() {
            width = line.len();
            height += 1;
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => {}
                    _ => match positions_by_frequency.entry(c) {
                        Entry::Occupied(positions) => positions.into_mut(),
                        Entry::Vacant(entry) => entry.insert(vec![]),
                    }
                    .push((x, y)),
                }
            }
        }
        AntennaMap {
            width,
            height,
            positions_by_frequency,
        }
    }
    pub fn count_unique_antinode_locations(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::day8::antenna_map::{AntennaMap, Point};
    use crate::input::input_to_string;
    use insta::assert_snapshot;
    use std::collections::HashMap;

    #[test]
    fn can_plot_example() {
        let input = input_to_string("day8/example.txt").unwrap();
        assert_snapshot!(plot_map(&AntennaMap::parse(input.as_str())));
    }

    fn plot_map(map: &AntennaMap) -> String {
        let antenna_index = index_antenna_by_position(map);
        let mut str = "".to_string();
        for y in 0..map.height {
            for x in 0..map.width {
                str.push(*antenna_index.get(&(x, y)).unwrap_or(&'.'));
            }
            str.push('\n');
        }
        str
    }
    fn index_antenna_by_position(map: &AntennaMap) -> HashMap<Point, char> {
        map.positions_by_frequency
            .iter()
            .flat_map(|(frequency, positions)| {
                positions.iter().map(|position| (*position, *frequency))
            })
            .collect()
    }
}
