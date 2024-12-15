const WALL: char = '#';
const BOX: char = 'O';
const EMPTY: char = '.';
const ROBOT: char = '@';

type Point = [usize; 2];

pub struct WarehouseMap {
    tiles: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl WarehouseMap {
    pub fn parse_map_and_robot_position(string: &str) -> (WarehouseMap, Point) {
        let map: Vec<Vec<char>> = string.lines().map(|line| line.chars().collect()).collect();
        let width = map.first().map(|line| line.len()).unwrap_or(0);
        let height = map.len();
        let robot_position = find_robot(&map, width, height);
        let tiles = map_walls_and_boxes(&map);
        (
            WarehouseMap {
                tiles,
                width,
                height,
            },
            robot_position,
        )
    }
    fn tile(&self, point: Point) -> char {
        let [x, y] = point;
        self.tiles[y][x]
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
mod tests {
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
        let (map, robot) = WarehouseMap::parse_map_and_robot_position(string);
        assert_eq!(
            print(&map),
            "\
            ########\n\
            #..O.O.#\n\
            ##..O..#\n\
            #...O..#\n\
            #.#.O..#\n\
            #...O..#\n\
            #......#\n\
            ########\n"
        );
        assert_eq!(robot, [2, 2]);
    }

    #[test]
    fn can_move_robot_up() {
        let string = "\
            ###\n\
            #.#\n\
            #@#\n";
        let map = WarehouseMap::parse_map_and_robot_position(string);
    }

    fn print(map: &WarehouseMap) -> String {
        let mut string = "".to_string();
        for y in 0..map.height {
            for x in 0..map.width {
                string.push(map.tile([x, y]));
            }
            string.push('\n');
        }
        string
    }
}
