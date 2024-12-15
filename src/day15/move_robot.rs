use crate::day15::warehouse::{Point, Warehouse};
use crate::day15::warehouse_box::WarehouseBox;
use std::collections::HashSet;

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

pub fn move_robot(warehouse: &Warehouse, direction: Direction) -> Warehouse {
    let new_position = next_point(warehouse.robot_position(), direction);
    let mut new_warehouse = warehouse.clone();
    if !warehouse.is_on_map(new_position) || warehouse.is_wall(new_position) {
        return new_warehouse;
    }
    match warehouse.box_at(new_position) {
        Some(b) => {
            if move_boxes(warehouse, &mut new_warehouse, b, direction) {
                new_warehouse.set_robot(new_position);
            }
        }
        None => new_warehouse.set_robot(new_position),
    }
    new_warehouse
}

fn move_boxes(
    warehouse: &Warehouse,
    new_warehouse: &mut Warehouse,
    first_box: &WarehouseBox,
    direction: Direction,
) -> bool {
    let mut boxes = vec![first_box];
    let mut to_move = HashSet::from([first_box.number()]);
    loop {
        let points: Vec<Point> = boxes
            .iter()
            .flat_map(|b| b.points_in_dir(direction))
            .map(|p| next_point(p, direction))
            .collect();
        boxes = vec![];
        for point in points {
            if let Some(b) = warehouse.box_at(point) {
                boxes.push(b);
            } else if !warehouse.is_on_map(point) || warehouse.is_wall(point) {
                return false;
            }
        }
        if boxes.is_empty() {
            new_warehouse.move_boxes(&to_move, direction);
            return true;
        }
        boxes.iter().map(|b| b.number()).for_each(|number| {
            to_move.insert(number);
        });
    }
}

fn next_point(point: Point, direction: Direction) -> Point {
    let [x, y] = point;
    match direction {
        Direction::Up => [x, y.wrapping_sub(1)],
        Direction::Down => [x, y + 1],
        Direction::Left => [x.wrapping_sub(1), y],
        Direction::Right => [x + 1, y],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day15::move_robot::Direction::{Down, Left, Right, Up};
    use crate::day15::warehouse::tests::print;

    #[test]
    fn can_move_robot() {
        let string = "\
            ...\n\
            .@.\n\
            ...\n";
        let before = Warehouse::parse(string);
        let after = [Up, Down, Left, Right].map(|d| print(&before.move_robot(d)));
        assert_eq!(
            after,
            [
                ".@.\n\
                 ...\n\
                 ...\n",
                "...\n\
                 ...\n\
                 .@.\n",
                "...\n\
                 @..\n\
                 ...\n",
                "...\n\
                 ..@\n\
                 ...\n"
            ]
        )
    }

    #[test]
    fn can_push_boxes() {
        let before = Warehouse::parse("@OO.");
        let after = before.move_robot(Right);
        assert_eq!(print(&after), ".@OO\n")
    }

    #[test]
    fn can_push_big_boxes_right() {
        let before = Warehouse::parse("@[][].");
        let after = before.move_robot(Right);
        assert_eq!(print(&after), ".@[][]\n")
    }

    #[test]
    fn can_push_big_boxes_left() {
        let before = Warehouse::parse(".[][]@");
        let after = before.move_robot(Left);
        assert_eq!(print(&after), "[][]@.\n")
    }

    #[test]
    fn can_push_big_boxes_down() {
        let string = "\
            .@..\n\
            .[].\n\
            []..\n\
            ..[]";
        let before = Warehouse::parse(string);
        let after = before.move_robot(Down);
        let expected = "\
            ....\n\
            .@..\n\
            .[].\n\
            [][]\n";
        assert_eq!(print(&after), expected)
    }

    #[test]
    fn can_push_chain_of_boxes() {
        let string = "\
            @...\n\
            []..\n\
            .[].\n\
            ..[]\n\
            ....";
        let before = Warehouse::parse(string);
        let after = before.move_robot(Down);
        let expected = "\
            ....\n\
            @...\n\
            []..\n\
            .[].\n\
            ..[]\n";
        assert_eq!(print(&after), expected)
    }

    #[test]
    fn can_stop_at_wall() {
        let before = Warehouse::parse("@#");
        let after = before.move_robot(Right);
        assert_eq!(print(&after), "@#\n")
    }

    #[test]
    fn can_stop_at_edge() {
        let before = Warehouse::parse("@");
        let after = before.move_robot(Right);
        assert_eq!(print(&after), "@\n")
    }

    #[test]
    fn can_stop_pushing_at_wall() {
        let before = Warehouse::parse("@O#");
        let after = before.move_robot(Right);
        assert_eq!(print(&after), "@O#\n")
    }

    #[test]
    fn can_stop_pushing_at_edge() {
        let before = Warehouse::parse("@O");
        let after = before.move_robot(Right);
        assert_eq!(print(&after), "@O\n")
    }
}
