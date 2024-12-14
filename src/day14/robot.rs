pub struct Robot {
    position: Position,
    velocity: Velocity,
}

pub type Position = [usize; 2];

pub type Velocity = [isize; 2];

pub type FloorSize = [usize; 2];

impl Robot {
    fn parse(_: &str) -> Robot {
        Robot {
            position: [0, 0],
            velocity: [0, 0],
        }
    }
    pub fn parse_vec(string: &str) -> Vec<Robot> {
        string.lines().map(Robot::parse).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::input_to_string;
    use insta::assert_snapshot;
    use std::collections::HashMap;

    #[test]
    fn can_print_robots() {
        let string = input_to_string("day14/example.txt").unwrap();
        let robots = Robot::parse_vec(&string);
        assert_snapshot!(print(&robots, [11, 7]))
    }

    fn print(robots: &Vec<Robot>, floor: FloorSize) -> String {
        let mut position_to_count: HashMap<Position, usize> = HashMap::new();
        for robot in robots {
            position_to_count
                .entry(robot.position)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let mut str = String::new();
        for y in 0..floor[1] {
            for x in 0..floor[0] {
                str.push_str(
                    &position_to_count
                        .get(&[x, y])
                        .map(|c| c.to_string())
                        .unwrap_or(".".to_string()),
                );
            }
            str.push('\n');
        }
        str
    }
}
