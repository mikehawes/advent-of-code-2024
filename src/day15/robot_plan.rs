use crate::day15::warehouse::Direction::{Down, Left, Right, Up};
use crate::day15::warehouse::{move_robot, Direction, Warehouse};

pub struct RobotPlan {
    warehouse: Warehouse,
    directions: Vec<Direction>,
}

impl RobotPlan {
    pub fn parse(string: &str) -> RobotPlan {
        let parts: Vec<&str> = string.split("\n\n").collect();
        let warehouse = Warehouse::parse(parts[0]);
        let directions = parse_directions(parts[1]);
        RobotPlan {
            warehouse,
            directions,
        }
    }
    pub fn sum_gps_coordinates_at_end(&self) -> usize {
        0
    }
    fn follow(&self) -> Warehouse {
        let mut warehouse = self.warehouse.clone();
        for direction in self.directions.iter() {
            warehouse = move_robot(&warehouse, *direction);
        }
        warehouse
    }
}

fn parse_directions(string: &str) -> Vec<Direction> {
    string
        .lines()
        .flat_map(|line| line.chars())
        .flat_map(to_direction)
        .collect()
}

fn to_direction(c: char) -> Option<Direction> {
    match c {
        '^' => Some(Up),
        '>' => Some(Right),
        'v' => Some(Down),
        '<' => Some(Left),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day15::warehouse;
    use crate::input::input_to_string;
    use insta::assert_snapshot;

    #[test]
    fn can_find_start() {
        let string = input_to_string("day15/small-example.txt").unwrap();
        let plan = RobotPlan::parse(&string);
        assert_snapshot!(print_at_start(&plan))
    }

    #[test]
    fn can_follow_plan() {
        let string = input_to_string("day15/small-example.txt").unwrap();
        let plan = RobotPlan::parse(&string);
        assert_snapshot!(print_warehouse(&plan.follow()))
    }

    #[test]
    fn can_follow_larger_plan() {
        let string = input_to_string("day15/example.txt").unwrap();
        let plan = RobotPlan::parse(&string);
        assert_snapshot!(print_warehouse(&plan.follow()))
    }

    fn print_at_start(plan: &RobotPlan) -> String {
        warehouse::tests::print(&plan.warehouse)
    }

    fn print_warehouse(warehouse: &Warehouse) -> String {
        warehouse::tests::print(warehouse)
    }
}
