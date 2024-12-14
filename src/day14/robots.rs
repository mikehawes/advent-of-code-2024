use crate::day14::robot::{move_for_seconds, print_robots, FloorSize, Robot};
use crate::day14::safety_factor::safety_factor;

pub struct Robots {
    robots: Vec<Robot>,
}

impl Robots {
    pub fn parse(string: &str) -> Robots {
        Robots {
            robots: Robot::parse_vec(string),
        }
    }
    pub fn safety_factor_after_seconds(&self, seconds: usize, floor: FloorSize) -> usize {
        let robots = move_for_seconds(&self.robots, floor, seconds);
        safety_factor(&robots, floor)
    }
    pub fn print_at_times<T>(
        &self,
        times: T,
        floor: FloorSize,
    ) -> impl Iterator<Item = (usize, String)> + use<'_, T>
    where
        T: Iterator<Item = usize>,
    {
        times.map(move |time| {
            (
                time,
                print_robots(&move_for_seconds(&self.robots, floor, time), floor),
            )
        })
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
        assert_eq!(robots.safety_factor_after_seconds(100, floor), 12)
    }
}
