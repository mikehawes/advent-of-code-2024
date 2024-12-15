use crate::day15::move_robot::{move_robot, Direction};
use crate::day15::warehouse_box::WarehouseBox;
use std::collections::{HashMap, HashSet};

pub const WALL: char = '#';
pub const BOX: char = 'O';
pub const BOX_LEFT: char = '[';
pub const BOX_RIGHT: char = ']';
pub const ROBOT: char = '@';

pub type Point = [usize; 2];

#[derive(Clone)]
pub struct Warehouse {
    width: usize,
    height: usize,
    robot_position: Point,
    boxes: Vec<WarehouseBox>,
    position_to_box_num: HashMap<Point, usize>,
    walls: HashSet<Point>,
}

impl Warehouse {
    pub fn parse(string: &str) -> Warehouse {
        let map: Vec<Vec<char>> = string.lines().map(|line| line.chars().collect()).collect();
        let width = map.first().map(|line| line.len()).unwrap_or(0);
        let height = map.len();
        let boxes = find_boxes(&map, width, height);
        let position_to_box_num = index_point_to_box_number(&boxes);
        Warehouse {
            width,
            height,
            walls: map_walls(&map, width, height),
            robot_position: find_robot(&map, width, height),
            boxes,
            position_to_box_num,
        }
    }
    pub fn move_robot(&self, direction: Direction) -> Warehouse {
        move_robot(self, direction)
    }
    pub fn robot_position(&self) -> Point {
        self.robot_position
    }
    pub fn sum_gps_coordinates(&self) -> usize {
        self.boxes
            .iter()
            .map(|b| b.position())
            .map(gps_coordinate)
            .sum()
    }
    pub fn scale_up(&self) -> Warehouse {
        let boxes = scale_up_boxes(&self.boxes);
        let position_to_box_num = index_point_to_box_number(&boxes);
        Warehouse {
            width: self.width * 2,
            height: self.height,
            walls: scale_up_walls(&self.walls),
            robot_position: scale_up_robot(self.robot_position),
            boxes,
            position_to_box_num,
        }
    }
    pub fn box_at(&self, point: Point) -> Option<&WarehouseBox> {
        self.position_to_box_num
            .get(&point)
            .map(|number| &self.boxes[*number])
    }
    pub fn is_wall(&self, point: Point) -> bool {
        self.walls.contains(&point)
    }
    pub fn is_on_map(&self, point: Point) -> bool {
        let [x, y] = point;
        x < self.width && y < self.height
    }
    pub fn set_robot(&mut self, position: Point) {
        self.robot_position = position;
    }
    pub fn move_boxes(&mut self, numbers: &HashSet<usize>, direction: Direction) {
        for number in numbers {
            for point in self.boxes[*number].points_iter() {
                self.position_to_box_num.remove(&point);
            }
        }
        for number in numbers {
            let b = self.boxes.get_mut(*number).unwrap();
            *b = b.move_dir(direction);
            for point in b.points_iter() {
                self.position_to_box_num.insert(point, *number);
            }
        }
    }
}

fn find_robot(tiles: &[Vec<char>], width: usize, height: usize) -> Point {
    (0..height)
        .flat_map(|y| (0..width).map(move |x| [x, y]))
        .find(|[x, y]| tiles[*y][*x] == ROBOT)
        .unwrap_or([width, height])
}

fn find_boxes(tiles: &[Vec<char>], width: usize, height: usize) -> Vec<WarehouseBox> {
    position_tiles(tiles, width, height)
        .filter(|(_, tile)| [BOX, BOX_LEFT].contains(tile))
        .enumerate()
        .map(|(number, (point, tile))| WarehouseBox::from_tile_at_point(number, tile, point))
        .collect()
}

fn index_point_to_box_number(boxes: &[WarehouseBox]) -> HashMap<Point, usize> {
    boxes
        .iter()
        .flat_map(|b| b.points_iter().map(|point| (point, b.number())))
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

fn scale_up_boxes(boxes: &[WarehouseBox]) -> Vec<WarehouseBox> {
    boxes.iter().map(|b| b.scale_up()).collect()
}

fn scale_up_robot(position: Point) -> Point {
    let [x, y] = position;
    [x * 2, y]
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::day15::warehouse_box;
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
            #...O..\n\
            #..@.O.\n";
        let warehouse = Warehouse::parse(string);
        assert_eq!(warehouse.sum_gps_coordinates(), 309)
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

    #[test]
    fn can_sum_gps_coordinates_scaled() {
        let string = "\
            #######\n\
            #...[].\n\
            #..@.[]\n";
        let warehouse = Warehouse::parse(string);
        assert_eq!(warehouse.sum_gps_coordinates(), 309)
    }

    pub fn print(warehouse: &Warehouse) -> String {
        let mut string = "".to_string();
        for y in 0..warehouse.height {
            for x in 0..warehouse.width {
                string.push(contents(warehouse, [x, y]));
            }
            string.push('\n');
        }
        string
    }

    fn contents(warehouse: &Warehouse, point: Point) -> char {
        if point == warehouse.robot_position() {
            ROBOT
        } else if warehouse.is_wall(point) {
            WALL
        } else if let Some(b) = warehouse.box_at(point) {
            warehouse_box::tests::char_at(b, point)
        } else {
            '.'
        }
    }
}
