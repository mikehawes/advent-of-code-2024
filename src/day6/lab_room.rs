use crate::day6::direction::Direction::Up;
use crate::day6::guard::Guard;
use std::collections::HashSet;

pub(super) type Point = (usize, usize);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LabRoom {
    width: usize,
    height: usize,
    obstructions: HashSet<Point>,
    guard: Guard,
}

#[derive(Debug, Eq, PartialEq)]
struct PathResult {
    positions: HashSet<Point>,
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
        self.check_path().num_positions()
    }
    pub fn count_positions_to_obstruct(&self) -> usize {
        let mut positions = 0;
        let unobstructed = self.check_path();
        for y in 0..self.height {
            println!("Checking line {y}");
            for x in 0..self.width {
                if self.guard.position == (x, y) {
                    continue;
                }
                if !unobstructed.positions.contains(&(x, y)) {
                    continue;
                }
                if self.obstructions.contains(&(x, y)) {
                    continue;
                }
                let mut copy = self.clone();
                copy.obstructions.insert((x, y));
                if copy.check_path().in_loop {
                    positions += 1;
                }
            }
        }
        positions
    }
    fn check_path(&self) -> PathResult {
        let mut positions = HashSet::new();
        let mut states = HashSet::new();
        let mut guard = self.guard.clone();
        positions.insert(guard.position);
        states.insert(guard.clone());
        loop {
            if !guard.move_forwards(self) {
                return PathResult::in_loop_after_visiting(positions);
            }
            if !self.is_in_room(guard.position) {
                return PathResult::left_after_visiting(positions);
            }
            if states.contains(&guard) {
                return PathResult::in_loop_after_visiting(positions);
            }
            positions.insert(guard.position);
            states.insert(guard.clone());
        }
    }
    fn is_in_room(&self, position: Point) -> bool {
        let (x, y) = position;
        x < self.width && y < self.height
    }
    pub(super) fn is_obstructed(&self, position: Point) -> bool {
        self.obstructions.contains(&position)
    }
}

impl PathResult {
    fn left_after_visiting(positions: HashSet<Point>) -> PathResult {
        PathResult {
            positions,
            in_loop: false,
        }
    }
    fn in_loop_after_visiting(positions: HashSet<Point>) -> PathResult {
        PathResult {
            positions,
            in_loop: true,
        }
    }
    fn num_positions(&self) -> usize {
        self.positions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day6::direction::Direction::Up;
    use crate::input::input_to_string;

    #[test]
    fn can_count_visited_positions_in_example() {
        let string = input_to_string("day6/example.txt").unwrap();
        let room = LabRoom::parse(string.as_str());
        assert_eq!(room.count_visited_positions(), 41)
    }

    #[test]
    fn can_count_positions_to_obstruct_in_example() {
        let string = input_to_string("day6/example.txt").unwrap();
        let room = LabRoom::parse(string.as_str());
        assert_eq!(room.count_positions_to_obstruct(), 6)
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
        assert_eq!(check_path_positions_and_is_loop(string), (1, false))
    }

    #[test]
    fn can_find_moved_forwards_twice() {
        let string = "\
               ..#\n\
               .^.";
        assert_eq!(check_path_positions_and_is_loop(string), (2, false))
    }

    #[test]
    fn can_find_moved_forwards_then_turned_right() {
        let string = "\
               ..#\n\
               ...\n\
               ..^";
        assert_eq!(check_path_positions_and_is_loop(string), (2, false))
    }

    #[test]
    fn can_find_moved_forwards_then_doubled_back() {
        let string = "\
               .#.\n\
               ..#\n\
               .^.";
        assert_eq!(check_path_positions_and_is_loop(string), (2, false))
    }

    #[test]
    fn can_find_fully_obstructed() {
        let string = "\
               .#.\n\
               #^#\n\
               .#.";
        assert_eq!(check_path_positions_and_is_loop(string), (1, true))
    }

    #[test]
    fn can_find_loop() {
        let string = "\
               .#..\n\
               .^.#\n\
               #...\n\
               ..#.";
        assert_eq!(check_path_positions_and_is_loop(string), (4, true))
    }

    fn check_path_positions_and_is_loop(string: &str) -> (usize, bool) {
        let result = LabRoom::parse(string).check_path();
        (result.positions.len(), result.in_loop)
    }
}
