use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

pub struct AntennaMap {
    width: usize,
    height: usize,
    positions_by_frequency: HashMap<char, Vec<Point>>,
}

pub(super) type Point = (usize, usize);
pub(super) type Vector = (isize, isize);

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
        let antinodes: HashSet<Point> = self.iter_antinodes().collect();
        antinodes.len()
    }
    pub fn count_unique_extended_antinode_locations(&self) -> usize {
        let antinodes: HashSet<Point> = self.iter_extended_antinodes().collect();
        antinodes.len()
    }
    fn iter_antinodes(&self) -> impl Iterator<Item = Point> + use<'_> {
        self.all_antenna_combinations()
            .flat_map(|(left, right)| antinodes_of_points(left, right))
            .filter(|point| self.is_in_grid(*point))
    }
    fn iter_extended_antinodes(&self) -> impl Iterator<Item = Point> + use<'_> {
        self.all_antenna_combinations()
            .flat_map(|(left, right)| self.extended_antinodes_of_points(left, right))
    }
    fn all_antenna_combinations(&self) -> impl Iterator<Item = (Point, Point)> + use<'_> {
        self.positions_by_frequency
            .values()
            .flat_map(all_combinations_for_frequency)
    }
    fn is_in_grid(&self, point: Point) -> bool {
        let (x, y) = point;
        x < self.width && y < self.height
    }
    fn extended_antinodes_of_points(&self, left: Point, right: Point) -> Vec<Point> {
        let (x1, y1) = to_vector(left);
        let (x2, y2) = to_vector(right);
        let xdiff = x2 - x1;
        let ydiff = y2 - y1;
        let mut antinodes = vec![left, right];
        self.extend_antinodes((x1, y1), (-xdiff, -ydiff), &mut antinodes);
        self.extend_antinodes((x2, y2), (xdiff, ydiff), &mut antinodes);
        antinodes
    }
    fn extend_antinodes(&self, origin: Vector, diff: Vector, antinodes: &mut Vec<Point>) {
        let (mut x, mut y) = origin;
        let (dx, dy) = diff;
        loop {
            x += dx;
            y += dy;
            let point = to_point((x, y));
            if self.is_in_grid(point) {
                antinodes.push(point);
            } else {
                return;
            }
        }
    }
}

fn all_combinations_for_frequency(
    positions: &Vec<Point>,
) -> impl Iterator<Item = (Point, Point)> + use<'_> {
    (0..positions.len())
        .flat_map(move |i| ((i + 1)..positions.len()).map(move |j| (positions[i], positions[j])))
}

fn antinodes_of_points(left: Point, right: Point) -> Vec<Point> {
    let (x1, y1) = to_vector(left);
    let (x2, y2) = to_vector(right);
    let xdiff = x2 - x1;
    let ydiff = y2 - y1;
    let point_0 = (x1 - xdiff, y1 - ydiff);
    let point_3 = (x2 + xdiff, y2 + ydiff);
    vec![to_point(point_0), to_point(point_3)]
}

fn to_vector(point: Point) -> Vector {
    let (left, right) = point;
    (left as isize, right as isize)
}

fn to_point(vector: Vector) -> Point {
    let (left, right) = vector;
    let a = 0_usize.wrapping_add_signed(left);
    let b = 0_usize.wrapping_add_signed(right);
    (a, b)
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
        assert_snapshot!(plot_map(input.as_str()));
    }

    #[test]
    fn can_plot_example_extended() {
        let input = input_to_string("day8/example.txt").unwrap();
        assert_snapshot!(plot_map_extended(input.as_str()));
    }

    #[test]
    fn can_count_antinodes_for_example() {
        let input = input_to_string("day8/example.txt").unwrap();
        let map = AntennaMap::parse(input.as_str());
        assert_eq!(map.count_unique_antinode_locations(), 14)
    }

    #[test]
    fn can_count_extendedantinodes_for_example() {
        let input = input_to_string("day8/example.txt").unwrap();
        let map = AntennaMap::parse(input.as_str());
        assert_eq!(map.count_unique_extended_antinode_locations(), 34)
    }

    #[test]
    fn can_find_antenna_combinations() {
        assert_eq!(
            all_antenna_combinations_by_number("aaa"),
            vec![(0, 1), (0, 2), (1, 2)]
        );
    }

    fn plot_map(map_string: &str) -> String {
        let map = AntennaMap::parse(map_string);
        let mut point_to_char = index_frequency_by_position(&map);
        map.iter_antinodes().for_each(|point| {
            point_to_char.entry(point).or_insert('#');
        });
        plot_map_with_chars(&map, point_to_char)
    }
    fn plot_map_extended(map_string: &str) -> String {
        let map = AntennaMap::parse(map_string);
        let mut point_to_char = index_frequency_by_position(&map);
        map.iter_extended_antinodes().for_each(|point| {
            point_to_char.entry(point).or_insert('#');
        });
        plot_map_with_chars(&map, point_to_char)
    }
    fn plot_map_with_chars(map: &AntennaMap, point_to_char: HashMap<Point, char>) -> String {
        let mut str = "".to_string();
        for y in 0..map.height {
            for x in 0..map.width {
                str.push(*point_to_char.get(&(x, y)).unwrap_or(&'.'));
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
