use crate::day14::robot::{FloorSize, Robot};

pub fn safety_factor(robots: &[Robot], floor: FloorSize) -> usize {
    let robots_top_left = robots
        .iter()
        .filter(|r| r.before_mid(floor, 0))
        .filter(|r| r.before_mid(floor, 1))
        .count();
    let robots_top_right = robots
        .iter()
        .filter(|r| r.after_mid(floor, 0))
        .filter(|r| r.before_mid(floor, 1))
        .count();
    let robots_bottom_left = robots
        .iter()
        .filter(|r| r.before_mid(floor, 0))
        .filter(|r| r.after_mid(floor, 1))
        .count();
    let robots_bottom_right = robots
        .iter()
        .filter(|r| r.after_mid(floor, 0))
        .filter(|r| r.after_mid(floor, 1))
        .count();
    robots_top_left * robots_top_right * robots_bottom_left * robots_bottom_right
}

#[cfg(test)]
mod tests {
    use crate::day14::robot::{move_for_seconds, Robot};
    use crate::day14::safety_factor::safety_factor;
    use crate::input::input_to_string;

    #[test]
    fn can_find_safety_factor() {
        let string = input_to_string("day14/example.txt").unwrap();
        let floor = [11, 7];
        let robots = move_for_seconds(&Robot::parse_vec(&string), floor, 100);
        assert_eq!(safety_factor(&robots, floor), 12)
    }
}
