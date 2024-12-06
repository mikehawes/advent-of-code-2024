use crate::day6::lab_room::Direction::Up;

type Point = (usize, usize);

#[derive(Debug, Eq, PartialEq)]
pub struct LabRoom {
    width: usize,
    height: usize,
    obstructions: Vec<Point>,
    guard: Guard,
}

#[derive(Debug, Eq, PartialEq)]
struct Guard {
    position: Point,
    direction: Direction,
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl LabRoom {
    pub fn count_visited_positions(&self) -> usize {
        0
    }
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
}
