const WALL: char = '#';
const BOX: char = 'O';
const EMPTY: char = '.';
const ROBOT: char = '@';

type Point = [usize; 2];

enum Direction {
    Up,
    Right,
    Left,
    Down,
}

#[derive(Clone)]
pub struct WarehouseMap {
    tiles: Vec<Vec<char>>,
    width: usize,
    height: usize,
    robot_position: Point,
}

impl WarehouseMap {
    pub fn parse(string: &str) -> WarehouseMap {
        let map: Vec<Vec<char>> = string.lines().map(|line| line.chars().collect()).collect();
        let width = map.first().map(|line| line.len()).unwrap_or(0);
        let height = map.len();
        let robot_position = find_robot(&map, width, height);
        let tiles = map_walls_and_boxes(&map);
        WarehouseMap {
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
}

fn move_robot(map: &WarehouseMap, direction: Direction) -> WarehouseMap {
    let [x, y] = map.robot_position;
    let new_position = match direction {
        Direction::Up => [x, y - 1],
        Direction::Down => [x, y + 1],
        Direction::Left => [x - 1, y],
        Direction::Right => [x + 1, y],
    };
    let mut new_map = map.clone();
    match map.tile(new_position) {
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

fn move_boxes(map: &mut WarehouseMap, point: Point, direction: Direction) -> bool {
    false
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
mod tests {
    use super::*;
    use crate::day15::warehouse_map::Direction::{Down, Left, Right, Up};

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
        let warehouse = WarehouseMap::parse(string);
        assert_eq!(print(&warehouse), string);
        assert_eq!(warehouse.robot_position, [2, 2]);
    }

    #[test]
    fn can_move_robot() {
        let string = "\
            ...\n\
            .@.\n\
            ...\n";
        let map = WarehouseMap::parse(string);
        let result = [Up, Down, Left, Right].map(|d| print(&move_robot(&map, d)));
        assert_eq!(
            result,
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
    #[ignore]
    fn can_push_boxes() {
        let map = WarehouseMap::parse("@OO.");
        let moved = move_robot(&map, Right);
        assert_eq!(print(&moved), ".@OO\n")
    }

    #[test]
    fn can_stop_at_wall() {
        let map = WarehouseMap::parse("@#");
        let moved = move_robot(&map, Right);
        assert_eq!(print(&moved), "@#\n")
    }

    fn print(map: &WarehouseMap) -> String {
        let mut string = "".to_string();
        for y in 0..map.height {
            for x in 0..map.width {
                string.push(map.contents([x, y]));
            }
            string.push('\n');
        }
        string
    }
}
