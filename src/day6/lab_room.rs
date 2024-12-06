use crate::day6::lab_room::Direction::{Down, Left, Right, Up};
use std::collections::HashSet;

type Point = (usize, usize);

#[derive(Debug, Eq, PartialEq)]
pub struct LabRoom {
    width: usize,
    height: usize,
    obstructions: HashSet<Point>,
    guard: Guard,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Guard {
    position: Point,
    direction: Direction,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug, Eq, PartialEq)]
struct PathResult {
    positions: usize,
    in_loop: bool,
}

impl LabRoom {
    pub fn parse(string: &str) -> LabRoom {
        let width = string.lines().next().unwrap().len();
        let height = string.lines().count();
        let mut obstructions = HashSet::new();
        let mut guard = Guard {
            position: (0, 0),
            direction: Up,
        };
        for (y, line) in string.lines().enumerate() {
            for (x, tile) in line.chars().enumerate() {
                if tile == '#' {
                    obstructions.insert((x, y));
                } else if tile == '^' {
                    guard.position = (x, y);
                }
            }
        }
        LabRoom {
            width,
            height,
            obstructions,
            guard,
        }
    }
    pub fn count_visited_positions(&self) -> usize {
        self.check_path().positions
    }
    fn check_path(&self) -> PathResult {
        let mut positions = HashSet::new();
        let mut states = HashSet::new();
        let mut guard = self.guard.clone();
        positions.insert(guard.position);
        states.insert(guard.clone());
        loop {
            if !guard.move_forwards(self) {
                return PathResult::in_loop_after_visiting(positions.len());
            }
            if !self.is_in_room(guard.position) {
                return PathResult::left_after_visiting(positions.len());
            }
            if states.contains(&guard) {
                return PathResult::in_loop_after_visiting(positions.len());
            }
            positions.insert(guard.position);
            states.insert(guard.clone());
        }
    }
    fn is_in_room(&self, position: Point) -> bool {
        let (x, y) = position;
        x < self.width && y < self.height
    }
    fn is_obstructed(&self, position: Point) -> bool {
        self.obstructions.contains(&position)
    }
}

impl Guard {
    fn move_forwards(&mut self, room: &LabRoom) -> bool {
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

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Up => Right,
            Left => Up,
            Right => Down,
            Down => Left,
        }
    }
}

impl PathResult {
    fn left_after_visiting(positions: usize) -> PathResult {
        PathResult {
            positions,
            in_loop: false,
        }
    }
    fn in_loop_after_visiting(positions: usize) -> PathResult {
        PathResult {
            positions,
            in_loop: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day6::lab_room::Direction::Up;
    use crate::input::input_to_string;

    #[test]
    fn can_count_positions_in_example() {
        let string = input_to_string("day6/example.txt").unwrap();
        let room = LabRoom::parse(string.as_str());
        assert_eq!(room.count_visited_positions(), 41)
    }

    #[test]
    fn can_parse_room() {
        let string = "\
               ..#\n\
               ..^";
        let room = LabRoom::parse(string);
        assert_eq!(
            room,
            LabRoom {
                width: 3,
                height: 2,
                obstructions: HashSet::from([(2, 0)]),
                guard: Guard {
                    position: (2, 1),
                    direction: Up
                }
            }
        )
    }

    #[test]
    fn can_find_moved_forwards_once() {
        let string = "\
               .^#\n\
               ...";
        assert_eq!(
            LabRoom::parse(string).check_path(),
            PathResult::left_after_visiting(1)
        )
    }

    #[test]
    fn can_find_moved_forwards_twice() {
        let string = "\
               ..#\n\
               .^.";
        assert_eq!(
            LabRoom::parse(string).check_path(),
            PathResult::left_after_visiting(2)
        )
    }

    #[test]
    fn can_find_moved_forwards_then_turned_right() {
        let string = "\
               ..#\n\
               ...\n\
               ..^";
        assert_eq!(
            LabRoom::parse(string).check_path(),
            PathResult::left_after_visiting(2)
        )
    }

    #[test]
    fn can_find_moved_forwards_then_doubled_back() {
        let string = "\
               .#.\n\
               ..#\n\
               .^.";
        assert_eq!(
            LabRoom::parse(string).check_path(),
            PathResult::left_after_visiting(2)
        )
    }

    #[test]
    fn can_find_fully_obstructed() {
        let string = "\
               .#.\n\
               #^#\n\
               .#.";
        assert_eq!(
            LabRoom::parse(string).check_path(),
            PathResult::in_loop_after_visiting(1)
        )
    }

    #[test]
    fn can_find_loop() {
        let string = "\
               .#..\n\
               .^.#\n\
               #...\n\
               ..#.";
        assert_eq!(
            LabRoom::parse(string).check_path(),
            PathResult::in_loop_after_visiting(4)
        )
    }
}
