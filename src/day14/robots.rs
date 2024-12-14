use crate::day14::robot::{FloorSize, Robot};

pub struct Robots {
    robots: Vec<Robot>,
}

impl Robots {
    pub fn parse(string: &str) -> Robots {
        Robots {
            robots: Robot::parse_vec(string),
        }
    }
    pub fn safety_factor_after_seconds(&self, _: usize, _: FloorSize) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::day14::robots::Robots;
    use crate::input::input_to_string;

    #[test]
    fn can_find_safety_factor_for_example() {
        let string = input_to_string("day14/example.txt").unwrap();
        let robots = Robots::parse(&string);
        let floor = [11, 7];
        assert_eq!(robots.safety_factor_after_seconds(100, floor), 0)
    }
}
