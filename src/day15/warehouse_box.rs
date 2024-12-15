use crate::day15::move_robot::Direction;
use crate::day15::warehouse::{Point, BOX_LEFT, BOX_RIGHT};

#[derive(Clone)]
pub struct WarehouseBox {
    number: usize,
    position: Point,
    width: usize,
}

impl WarehouseBox {
    pub fn from_tile_at_point(number: usize, tile: char, point: Point) -> WarehouseBox {
        let [x, y] = point;
        match tile {
            BOX_LEFT => WarehouseBox {
                number,
                position: point,
                width: 2,
            },
            BOX_RIGHT => WarehouseBox {
                number,
                position: [x - 1, y],
                width: 2,
            },
            _ => WarehouseBox {
                number,
                position: point,
                width: 1,
            },
        }
    }
    pub fn points_in_dir(&self, direction: Direction) -> Vec<Point> {
        let [x, y] = self.position;
        match direction {
            Direction::Left => vec![[x, y]],
            Direction::Right => vec![[x + self.width - 1, y]],
            _ => (0..self.width).map(move |i| [x + i, y]).collect(),
        }
    }
    pub fn points_iter(&self) -> impl Iterator<Item = Point> {
        let [x, y] = self.position;
        (0..self.width).map(move |i| [x + i, y])
    }
    pub fn number(&self) -> usize {
        self.number
    }
    pub fn scale_up(&self) -> WarehouseBox {
        let [x, y] = self.position;
        WarehouseBox {
            number: self.number,
            position: [x * 2, y],
            width: self.width * 2,
        }
    }
    pub fn char_at(&self, point: Point) -> char {
        if self.width == 1 {
            return 'O';
        }
        let offset = point[0] - self.position[0];
        if offset == 0 {
            '['
        } else {
            ']'
        }
    }
    pub fn move_dir(&self, direction: Direction) -> WarehouseBox {
        let [x, y] = self.position;
        let position = match direction {
            Direction::Up => [x, y - 1],
            Direction::Down => [x, y + 1],
            Direction::Left => [x - 1, y],
            Direction::Right => [x + 1, y],
        };
        WarehouseBox {
            number: self.number,
            position,
            width: self.width,
        }
    }
}
