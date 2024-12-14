use regex::Regex;
use std::str::FromStr;

pub struct Robot {
    position: Position,
    velocity: Velocity,
}

pub type Position = [usize; 2];

pub type Velocity = [isize; 2];

pub type FloorSize = [usize; 2];

impl Robot {
    pub fn parse_vec(string: &str) -> Vec<Robot> {
        let regex = Regex::new(r"p=([0-9]+),([0-9]+) v=([\-0-9]+),([\-0-9]+)").unwrap();
        string
            .lines()
            .map(|line| Self::parse(line, &regex))
            .collect()
    }
    fn parse(string: &str, regex: &Regex) -> Robot {
        let captures = regex.captures(string).unwrap();
        let x = usize::from_str(&captures[1]).unwrap();
        let y = usize::from_str(&captures[2]).unwrap();
        let vx = isize::from_str(&captures[3]).unwrap();
        let vy = isize::from_str(&captures[4]).unwrap();
        Robot {
            position: [x, y],
            velocity: [vx, vy],
        }
    }
}

pub fn move_for_seconds(robots: &[Robot], floor: FloorSize, seconds: usize) -> Vec<Robot> {
    let width = floor[0] as isize;
    let height = floor[1] as isize;
    robots
        .iter()
        .map(|robot| {
            let [x, y] = robot.position;
            let [vx, vy] = robot.velocity;
            let new_x_wrapping = x as isize + seconds as isize * vx;
            let new_y_wrapping = y as isize + seconds as isize * vy;
            let new_x = wrapping_to_pos(new_x_wrapping, width);
            let new_y = wrapping_to_pos(new_y_wrapping, height);
            Robot {
                position: [new_x, new_y],
                velocity: robot.velocity,
            }
        })
        .collect()
}

fn wrapping_to_pos(wrapping: isize, length: isize) -> usize {
    let remainder = wrapping % length;
    if remainder >= 0 {
        remainder as usize
    } else {
        (length + remainder) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::input_to_string;
    use insta::assert_snapshot;
    use std::collections::HashMap;

    #[test]
    fn can_parse_robots() {
        let string = input_to_string("day14/example.txt").unwrap();
        let robots = Robot::parse_vec(&string);
        let floor = [11, 7];
        assert_snapshot!(print(&robots, floor))
    }

    #[test]
    fn can_move_for_100_seconds() {
        let string = input_to_string("day14/example.txt").unwrap();
        let robots = Robot::parse_vec(&string);
        let floor = [11, 7];
        let after = move_for_seconds(&robots, floor, 100);
        assert_snapshot!(print(&after, floor))
    }

    fn print(robots: &Vec<Robot>, floor: FloorSize) -> String {
        let position_to_count = index_position_to_count(robots);
        let mut str = String::new();
        for y in 0..floor[1] {
            for x in 0..floor[0] {
                str.push_str(
                    &position_to_count
                        .get(&[x, y])
                        .map(print_count)
                        .unwrap_or(".".to_string()),
                );
            }
            str.push('\n');
        }
        str
    }

    fn index_position_to_count(robots: &Vec<Robot>) -> HashMap<Position, usize> {
        let mut position_to_count: HashMap<Position, usize> = HashMap::new();
        for robot in robots {
            position_to_count
                .entry(robot.position)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        position_to_count
    }

    fn print_count(count: &usize) -> String {
        if *count > 9 {
            "X".to_string()
        } else {
            count.to_string()
        }
    }
}
