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
    fn all_antenna_combinations(&self) -> impl Iterator<Item = (Point, Point)> + use<'_> {
        self.positions_by_frequency
            .values()
            .flat_map(all_combinations_for_frequency)
    }
}

fn all_combinations_for_frequency(
    positions: &Vec<Point>,
) -> impl Iterator<Item = (Point, Point)> + use<'_> {
    (0..positions.len())
        .flat_map(move |i| ((i + 1)..positions.len()).map(move |j| (positions[i], positions[j])))
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

    #[test]
    fn can_find_antenna_combinations() {
        assert_eq!(
            all_antenna_combinations_by_number("aaa"),
            vec![(0, 1), (0, 2), (1, 2)]
        );
    }

    fn plot_map(map: &AntennaMap) -> String {
        let antenna_index = index_frequency_by_position(map);
        let mut str = "".to_string();
        for y in 0..map.height {
            for x in 0..map.width {
                str.push(*antenna_index.get(&(x, y)).unwrap_or(&'.'));
            }
            str.push('\n');
        }
        str
    }
    fn index_frequency_by_position(map: &AntennaMap) -> HashMap<Point, char> {
        map.positions_by_frequency
            .iter()
            .flat_map(|(frequency, positions)| {
                positions.iter().map(|position| (*position, *frequency))
            })
            .collect()
    }
    fn all_antenna_combinations_by_number(map_string: &str) -> Vec<(usize, usize)> {
        let map = AntennaMap::parse(map_string);
        let number_by_position = index_antenna_number_by_position(&map);
        map.all_antenna_combinations()
            .map(|(left, right)| {
                (
                    *number_by_position.get(&left).unwrap(),
                    *number_by_position.get(&right).unwrap(),
                )
            })
            .collect()
    }
    fn index_antenna_number_by_position(map: &AntennaMap) -> HashMap<Point, usize> {
        map.positions_by_frequency
            .values()
            .flat_map(|positions| positions.iter())
            .enumerate()
            .map(|(i, position)| (*position, i))
            .collect()
    }
}
