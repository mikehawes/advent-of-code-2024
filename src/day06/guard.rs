use crate::day06::direction::Direction;
use crate::day06::direction::Direction::{Down, Left, Right, Up};
use crate::day06::lab_room::{LabRoom, Point};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Guard {
    pub position: Point,
    pub direction: Direction,
}

impl Guard {
    pub fn move_forwards(&mut self, room: &LabRoom) -> bool {
        let mut direction = self.direction;
        for _ in 0..4 {
            let next = self.next_position(direction);
            if !room.is_obstructed(next) {
                self.position = next;
                self.direction = direction;
                return true;
            }
            direction = direction.turn_right();
        }
        false
    }
    fn next_position(&self, direction: Direction) -> Point {
        let (x, y) = self.position;
        match direction {
            Up => (x, y.wrapping_sub(1)),
            Left => (x.wrapping_sub(1), y),
            Right => (x + 1, y),
            Down => (x, y + 1),
        }
    }
}
