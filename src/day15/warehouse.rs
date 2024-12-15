const WALL: char = '#';
const BOX: char = 'O';
const EMPTY: char = '.';
const ROBOT: char = '@';

type Point = [usize; 2];

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

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
    fn contents(&self, point: Point) -> char {
        if point == self.robot_position {
            ROBOT
        } else {
            self.tile(point)
        }
    }
    fn tile(&self, point: Point) -> char {
        let [x, y] = point;
        self.tiles[y][x]
    }
    fn is_on_map(&self, point: Point) -> bool {
        let [x, y] = point;
        x < self.width && y < self.height
    }
    fn write_tile(&mut self, point: Point, contents: char) {
        let [x, y] = point;
        self.tiles[y][x] = contents;
    }
}

pub fn move_robot(warehouse: &Warehouse, direction: Direction) -> Warehouse {
    let new_position = next_point(warehouse.robot_position, direction);
    let mut new_map = warehouse.clone();
    if !warehouse.is_on_map(new_position) {
        return new_map;
    }
    match warehouse.tile(new_position) {
        BOX => {
            if move_boxes(&mut new_map, new_position, direction) {
                new_map.robot_position = new_position;
            }
        }
        WALL => {}
        _ => new_map.robot_position = new_position,
    }
    new_map
}

fn move_boxes(warehouse: &mut Warehouse, first_box: Point, direction: Direction) -> bool {
    let mut point = first_box;
    loop {
        point = next_point(point, direction);
        if !warehouse.is_on_map(point) {
            return false;
        }
        match warehouse.tile(point) {
            BOX => {}
            WALL => return false,
            _ => {
                warehouse.write_tile(point, BOX);
                warehouse.write_tile(first_box, EMPTY);
                return true;
            }
        }
    }
}

fn next_point(point: Point, direction: Direction) -> Point {
    let [x, y] = point;
    match direction {
        Direction::Up => [x, y.wrapping_sub(1)],
        Direction::Down => [x, y + 1],
        Direction::Left => [x.wrapping_sub(1), y],
        Direction::Right => [x + 1, y],
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::day15::warehouse::Direction::{Down, Left, Right, Up};

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
    fn can_move_robot() {
        let string = "\
            ...\n\
            .@.\n\
            ...\n";
        let before = Warehouse::parse(string);
        let after = [Up, Down, Left, Right].map(|d| print(&move_robot(&before, d)));
        assert_eq!(
            after,
            [
                ".@.\n\
                 ...\n\
                 ...\n",
                "...\n\
                 ...\n\
                 .@.\n",
                "...\n\
                 @..\n\
                 ...\n",
                "...\n\
                 ..@\n\
                 ...\n"
            ]
        )
    }

    #[test]
    fn can_push_boxes() {
        let before = Warehouse::parse("@OO.");
        let after = move_robot(&before, Right);
        assert_eq!(print(&after), ".@OO\n")
    }

    #[test]
    fn can_stop_at_wall() {
        let before = Warehouse::parse("@#");
        let after = move_robot(&before, Right);
        assert_eq!(print(&after), "@#\n")
    }

    #[test]
    fn can_stop_at_edge() {
        let before = Warehouse::parse("@");
        let after = move_robot(&before, Right);
        assert_eq!(print(&after), "@\n")
    }

    #[test]
    fn can_stop_pushing_at_wall() {
        let before = Warehouse::parse("@O#");
        let after = move_robot(&before, Right);
        assert_eq!(print(&after), "@O#\n")
    }

    #[test]
    fn can_stop_pushing_at_edge() {
        let before = Warehouse::parse("@O");
        let after = move_robot(&before, Right);
        assert_eq!(print(&after), "@O\n")
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
