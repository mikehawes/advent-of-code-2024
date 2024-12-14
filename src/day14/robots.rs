use crate::day14::robot::{FloorSize, Robot};

pub struct Robots {
    robots: Vec<Robot>,
}

impl Robots {
    pub fn parse(string: &str) -> Robots {
        let robots = string.lines().map(Robot::parse).collect();
        Robots { robots }
    }
    pub fn safety_factor_after_seconds(&self, _: usize, _: FloorSize) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::day14::robot;
    use crate::day14::robot::FloorSize;
    use crate::day14::robots::Robots;
    use crate::input::input_to_string;
    use insta::assert_snapshot;

    #[test]
    fn can_print_robots() {
        let string = input_to_string("day14/example.txt").unwrap();
        let robots = Robots::parse(&string);
        assert_snapshot!(print(&robots, [11, 7]))
    }

    fn print(robots: &Robots, floor: FloorSize) -> String {
        robot::tests::print(&robots.robots, floor)
    }
}
