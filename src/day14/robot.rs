use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Robot {
    position: Position,
    velocity: Velocity,
}

pub type Position = [usize; 2];

pub type Velocity = [isize; 2];

pub type FloorSize = [usize; 2];

pub type SignedFloorSize = [isize; 2];

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
    pub fn before_mid(&self, floor: FloorSize, dimension: usize) -> bool {
        self.position[dimension] < floor[dimension] / 2
    }
    pub fn after_mid(&self, floor: FloorSize, dimension: usize) -> bool {
        self.position[dimension] > floor[dimension] / 2
    }
}

pub fn move_for_seconds(robots: &[Robot], floor: FloorSize, seconds: usize) -> Vec<Robot> {
    let signed_floor = signed_floor_size(floor);
    robots
        .iter()
        .map(|robot| move_robot_for_seconds(robot, &signed_floor, seconds))
        .collect()
}

fn move_robot_for_seconds(robot: &Robot, floor: &SignedFloorSize, seconds: usize) -> Robot {
    let mut position = [0, 0];
    for i in 0..2 {
        position[i] =
            move_dimension_for_seconds(robot.position[i], robot.velocity[i], floor[i], seconds);
    }
    Robot {
        position,
        velocity: robot.velocity,
    }
}

fn move_dimension_for_seconds(
    position: usize,
    velocity: isize,
    length: isize,
    seconds: usize,
) -> usize {
    let wrapping = position as isize + seconds as isize * velocity;
    let remainder = wrapping % length;
    if remainder >= 0 {
        remainder as usize
    } else {
        (length + remainder) as usize
    }
}

fn signed_floor_size(floor_size: FloorSize) -> SignedFloorSize {
    let [width, height] = floor_size;
    [width as isize, height as isize]
}

pub fn print_robots(robots: &Vec<Robot>, floor: FloorSize) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::input_to_string;
    use insta::assert_snapshot;

    #[test]
    fn can_parse_robots() {
        let string = input_to_string("day14/example.txt").unwrap();
        let robots = Robot::parse_vec(&string);
        let floor = [11, 7];
        assert_snapshot!(print_robots(&robots, floor))
    }

    #[test]
    fn can_move_for_100_seconds() {
        let string = input_to_string("day14/example.txt").unwrap();
        let robots = Robot::parse_vec(&string);
        let floor = [11, 7];
        let after = move_for_seconds(&robots, floor, 100);
        assert_snapshot!(print_robots(&after, floor))
    }
}
