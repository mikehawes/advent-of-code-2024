use crate::day15::move_robot::{move_robot, Direction};

pub const WALL: char = '#';
pub const BOX: char = 'O';
pub const EMPTY: char = '.';
pub const ROBOT: char = '@';

pub type Point = [usize; 2];

#[derive(Clone)]
pub struct Warehouse {
    tiles: Vec<Vec<char>>,
    width: usize,
    height: usize,
    robot_position: Point,
}

impl Warehouse {
    pub fn parse(string: &str) -> Warehouse {
        let map: Vec<Vec<char>> = string.lines().map(|line| line.chars().collect()).collect();
        let width = map.first().map(|line| line.len()).unwrap_or(0);
        let height = map.len();
        let robot_position = find_robot(&map, width, height);
        let tiles = map_walls_and_boxes(&map);
        Warehouse {
            tiles,
            width,
            height,
            robot_position,
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
    fn contents(&self, point: Point) -> char {
        if point == self.robot_position {
            ROBOT
        } else {
            self.tile(point)
        }
    }
    pub fn tile(&self, point: Point) -> char {
        let [x, y] = point;
        self.tiles[y][x]
    }
    pub fn is_on_map(&self, point: Point) -> bool {
        let [x, y] = point;
        x < self.width && y < self.height
    }
    pub fn set_robot(&mut self, position: Point) {
        self.robot_position = position;
    }
    pub fn write_tile(&mut self, point: Point, contents: char) {
        let [x, y] = point;
        self.tiles[y][x] = contents;
    }
}

fn find_robot(tiles: &[Vec<char>], width: usize, height: usize) -> Point {
    (0..height)
        .flat_map(|y| (0..width).map(move |x| [x, y]))
        .find(|[x, y]| tiles[*y][*x] == ROBOT)
        .unwrap_or([width, height])
}

fn map_walls_and_boxes(tiles: &[Vec<char>]) -> Vec<Vec<char>> {
    tiles
        .iter()
        .map(|line| {
            line.iter()
                .map(|tile| match *tile {
                    WALL => WALL,
                    BOX => BOX,
                    _ => EMPTY,
                })
                .collect()
        })
        .collect()
}

fn gps_coordinate(point: Point) -> usize {
    point[1] * 100 + point[0]
}

#[cfg(test)]
pub mod tests {
    use super::*;

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
