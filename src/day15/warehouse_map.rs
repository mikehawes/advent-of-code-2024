pub struct WarehouseMap {
    tiles: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl WarehouseMap {
    pub fn parse(string: &str) -> WarehouseMap {
        let tiles: Vec<Vec<char>> = string.lines().map(|line| line.chars().collect()).collect();
        let width = tiles.first().map(|line| line.len()).unwrap_or(0);
        let height = tiles.len();
        WarehouseMap {
            tiles,
            width,
            height,
        }
    }
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
        let map = WarehouseMap::parse(string);
        assert_eq!(print(&map), string)
    }

    fn print(map: &WarehouseMap) -> String {
        let mut string = "".to_string();
        for y in 0..map.height {
            for x in 0..map.width {
                string.push(map.tiles[y][x]);
            }
            string.push('\n');
        }
        string
    }
}
