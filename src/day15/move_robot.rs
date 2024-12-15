use crate::day15::warehouse::{Point, Warehouse, BOX, BOX_LEFT, BOX_RIGHT, EMPTY, WALL};

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

pub fn move_robot(warehouse: &Warehouse, direction: Direction) -> Warehouse {
    let new_position = next_point(warehouse.robot_position(), direction);
    let mut new_map = warehouse.clone();
    if !warehouse.is_on_map(new_position) {
        return new_map;
    }
    match warehouse.tile(new_position) {
        BOX | BOX_LEFT | BOX_RIGHT => {
            if move_boxes(&mut new_map, new_position, direction) {
                new_map.set_robot(new_position);
            }
        }
        WALL => {}
        _ => new_map.set_robot(new_position),
    }
    new_map
}

fn move_boxes(warehouse: &mut Warehouse, first_box: Point, direction: Direction) -> bool {
    let mut point = first_box;
    loop {
        point = next_point(point, direction);
        if !warehouse.is_on_map(point) {
            return false;
        }
        match warehouse.tile(point) {
            BOX | BOX_LEFT | BOX_RIGHT => {}
            WALL => return false,
            _ => loop {
                let write_to = point;
                point = prev_point(point, direction);
                warehouse.write_tile(write_to, warehouse.tile(point));
                if point == first_box {
                    warehouse.write_tile(first_box, EMPTY);
                    return true;
                }
            },
        }
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

fn prev_point(point: Point, direction: Direction) -> Point {
    let [x, y] = point;
    match direction {
        Direction::Up => [x, y + 1],
        Direction::Down => [x, y.wrapping_sub(1)],
        Direction::Left => [x + 1, y],
        Direction::Right => [x.wrapping_sub(1), y],
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
    #[ignore]
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
            [][]";
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
