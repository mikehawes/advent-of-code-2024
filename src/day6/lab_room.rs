use crate::day6::lab_room::Direction::Up;

type Point = (usize, usize);

#[derive(Debug, Eq, PartialEq)]
pub struct LabRoom {
    width: usize,
    height: usize,
    obstructions: Vec<Point>,
    guard: Guard,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Guard {
    position: Point,
    direction: Direction,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl LabRoom {
    pub fn parse(string: &str) -> LabRoom {
        let width = string.lines().next().unwrap().len();
        let height = string.lines().count();
        let mut obstructions = vec![];
        let mut guard = Guard {
            position: (0, 0),
            direction: Up,
        };
        for (y, line) in string.lines().enumerate() {
            for (x, tile) in line.chars().enumerate() {
                if tile == '#' {
                    obstructions.push((x, y));
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
        let mut positions = 1;
        let mut guard = self.guard.clone();
        while guard.move_forwards(self) {
            positions += 1;
        }
        positions
    }
    fn is_in_room(&self, guard: &Guard) -> bool {
        let (x, y) = guard.position;
        x < self.width && y < self.height
    }
}

impl Guard {
    fn move_forwards(&mut self, room: &LabRoom) -> bool {
        let (x, y) = self.position;
        self.position = (x, y.wrapping_sub(1));
        room.is_in_room(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day6::lab_room::Direction::Up;

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
                obstructions: vec![(2, 0)],
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
        assert_eq!(LabRoom::parse(string).count_visited_positions(), 1)
    }

    #[test]
    fn can_find_moved_forwards_twice() {
        let string = "\
               ..#\n\
               .^.";
        assert_eq!(LabRoom::parse(string).count_visited_positions(), 2)
    }

    #[test]
    #[ignore]
    fn can_find_moved_forwards_then_turned_right() {
        let string = "\
               ..#\n\
               ...\n\
               ..^";
        assert_eq!(LabRoom::parse(string).count_visited_positions(), 2)
    }
}
