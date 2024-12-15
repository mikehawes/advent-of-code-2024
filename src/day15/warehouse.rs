use crate::day15::move_robot::{move_robot, Direction};
use std::collections::{HashMap, HashSet};

pub const WALL: char = '#';
pub const BOX: char = 'O';
pub const BOX_LEFT: char = '[';
pub const BOX_RIGHT: char = ']';
pub const EMPTY: char = '.';
pub const ROBOT: char = '@';

pub type Point = [usize; 2];

#[derive(Clone)]
pub struct Warehouse {
    width: usize,
    height: usize,
    robot_position: Point,
    box_position_to_char: HashMap<Point, char>,
    walls: HashSet<Point>,
}

impl Warehouse {
    pub fn parse(string: &str) -> Warehouse {
        let map: Vec<Vec<char>> = string.lines().map(|line| line.chars().collect()).collect();
        let width = map.first().map(|line| line.len()).unwrap_or(0);
        let height = map.len();
        Warehouse {
            width,
            height,
            walls: map_walls(&map, width, height),
            robot_position: find_robot(&map, width, height),
            box_position_to_char: find_boxes(&map, width, height),
        }
    }
    pub fn move_robot(&self, direction: Direction) -> Warehouse {
        move_robot(self, direction)
    }
    pub fn robot_position(&self) -> Point {
        self.robot_position
    }
    pub fn sum_gps_coordinates(&self) -> usize {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| [x, y]))
            .filter(|point| self.contents(*point) == BOX)
            .map(gps_coordinate)
            .sum()
    }
    pub fn scale_up(&self) -> Warehouse {
        Warehouse {
            walls: scale_up_walls(&self.walls),
            width: self.width * 2,
            height: self.height,
            robot_position: scale_up_robot(self.robot_position),
            box_position_to_char: scale_up_boxes(&self.box_position_to_char),
        }
    }
    fn contents(&self, point: Point) -> char {
        if point == self.robot_position {
            ROBOT
        } else {
            self.tile(point)
        }
    }
    pub fn tile(&self, point: Point) -> char {
        *self.box_position_to_char.get(&point).unwrap_or_else(|| {
            if self.walls.contains(&point) {
                &WALL
            } else {
                &EMPTY
            }
        })
    }
    pub fn is_on_map(&self, point: Point) -> bool {
        let [x, y] = point;
        x < self.width && y < self.height
    }
    pub fn set_robot(&mut self, position: Point) {
        self.robot_position = position;
    }
    pub fn write_tile(&mut self, point: Point, contents: char) {
        match contents {
            BOX | BOX_LEFT | BOX_RIGHT => {
                self.box_position_to_char.insert(point, contents);
            }
            EMPTY => {
                self.box_position_to_char.remove(&point);
            }
            _ => {}
        }
    }
}

fn find_robot(tiles: &[Vec<char>], width: usize, height: usize) -> Point {
    (0..height)
        .flat_map(|y| (0..width).map(move |x| [x, y]))
        .find(|[x, y]| tiles[*y][*x] == ROBOT)
        .unwrap_or([width, height])
}

fn find_boxes(tiles: &[Vec<char>], width: usize, height: usize) -> HashMap<Point, char> {
    position_tiles(tiles, width, height)
        .filter(|(_, tile)| [BOX, BOX_LEFT, BOX_RIGHT].contains(tile))
        .collect()
}

fn map_walls(tiles: &[Vec<char>], width: usize, height: usize) -> HashSet<Point> {
    position_tiles(tiles, width, height)
        .filter(|(_, tile)| WALL == *tile)
        .map(|(point, _)| point)
        .collect()
}

fn position_tiles(
    tiles: &[Vec<char>],
    width: usize,
    height: usize,
) -> impl Iterator<Item = (Point, char)> + use<'_> {
    positions(width, height).map(|[x, y]| ([x, y], tiles[y][x]))
}

fn positions(width: usize, height: usize) -> impl Iterator<Item = Point> {
    (0..height).flat_map(move |y| (0..width).map(move |x| [x, y]))
}

fn gps_coordinate(point: Point) -> usize {
    point[1] * 100 + point[0]
}

fn scale_up_walls(walls: &HashSet<Point>) -> HashSet<Point> {
    walls
        .iter()
        .flat_map(|[x, y]| [[*x * 2, *y], [*x * 2 + 1, *y]])
        .collect()
}

fn scale_up_boxes(box_position_to_char: &HashMap<Point, char>) -> HashMap<Point, char> {
    box_position_to_char
        .keys()
        .flat_map(|[x, y]| [([*x * 2, *y], BOX_LEFT), ([*x * 2 + 1, *y], BOX_RIGHT)])
        .collect()
}

fn scale_up_robot(position: Point) -> Point {
    let [x, y] = position;
    [x * 2, y]
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn can_parse_map() {
        let string = "\
            ########\n\
            #..O.O.#\n\
            ##@.O..#\n\
            #...O..#\n\
            #.#.O..#\n\
            #...O..#\n\
            #......#\n\
            ########\n";
        let warehouse = Warehouse::parse(string);
        assert_eq!(print(&warehouse), string);
        assert_eq!(warehouse.robot_position, [2, 2]);
    }

    #[test]
    fn can_find_gps_coordinate() {
        assert_eq!(gps_coordinate([4, 1]), 104)
    }

    #[test]
    fn can_sum_gps_coordinates() {
        let string = "\
            #######\n\
            #...O.O\n\
            #..@...\n";
        let warehouse = Warehouse::parse(string);
        assert_eq!(warehouse.sum_gps_coordinates(), 210)
    }

    #[test]
    fn can_scale_up() {
        let string = "\
            ########\n\
            #..O.O.#\n\
            ##@.O..#\n\
            #...O..#\n\
            #.#.O..#\n\
            #...O..#\n\
            #......#\n\
            ########\n";
        let warehouse = Warehouse::parse(string);
        assert_snapshot!(print(&warehouse.scale_up()))
    }

    pub fn print(warehouse: &Warehouse) -> String {
        let mut string = "".to_string();
        for y in 0..warehouse.height {
            for x in 0..warehouse.width {
                string.push(warehouse.contents([x, y]));
            }
            string.push('\n');
        }
        string
    }
}
