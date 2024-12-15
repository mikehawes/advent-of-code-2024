use crate::day15::warehouse_map::WarehouseMap;

pub struct RobotPlan {
    warehouse: WarehouseMap,
}

impl RobotPlan {
    pub fn parse(string: &str) -> RobotPlan {
        let parts: Vec<&str> = string.split("\n\n").collect();
        let warehouse = WarehouseMap::parse(parts[0]);
        RobotPlan { warehouse }
    }
    pub fn sum_gps_coordinates_at_end(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day15::warehouse_map;
    use crate::input::input_to_string;
    use insta::assert_snapshot;

    #[test]
    fn can_find_start() {
        let string = input_to_string("day15/small-example.txt").unwrap();
        let plan = RobotPlan::parse(&string);
        assert_snapshot!(print_at_start(&plan))
    }

    fn print_at_start(plan: &RobotPlan) -> String {
        warehouse_map::tests::print(&plan.warehouse)
    }
}
