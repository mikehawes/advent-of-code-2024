pub type Point = [usize; 2];

pub fn find_units_along_each_line(line_1: &[Point; 2], line_2: &[Point; 2]) -> Option<[usize; 2]> {
    let [[x1, y1], [x2, y2]] = to_signed(line_1);
    let [[x3, y3], [x4, y4]] = to_signed(line_2);
    let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if denominator == 0 {
        return None;
    }
    let t_numerator = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
    let u_numerator = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3));
    if t_numerator % denominator != 0 || u_numerator % denominator != 0 {
        return None;
    }
    let t = t_numerator / denominator;
    let u = u_numerator / denominator;
    if t < 0 || u < 0 {
        return None;
    }
    Some([t as usize, u as usize])
}

type SignedLine = [[isize; 2]; 2];

fn to_signed(line: &[Point; 2]) -> SignedLine {
    let [x1, y1] = line[0];
    let [x2, y2] = line[1];
    [[x1 as isize, y1 as isize], [x2 as isize, y2 as isize]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_lines_stop_at_same_point() {
        assert_eq!(
            find_units_along_each_line(&[[0, 0], [1, 1]], &[[2, 0], [2, 1]]),
            Some([2, 2])
        );
    }

    #[test]
    fn can_find_two_a_presses_then_one_b_press() {
        let a = [1, 1];
        let b = [0, 1];
        let prize = [2, 3];
        let line_1 = [[0, 0], a];
        let line_2 = [prize, [prize[0] - b[0], prize[1] - b[1]]];
        assert_eq!(find_units_along_each_line(&line_1, &line_2), Some([2, 1]));
    }

    #[test]
    fn can_find_lines_cross_on_a_point_where_they_do_not_stop() {
        assert_eq!(
            find_units_along_each_line(&[[0, 0], [2, 2]], &[[0, 2], [2, 0]]),
            None
        );
    }

    #[test]
    fn can_find_parallel_lines_never_touch() {
        assert_eq!(
            find_units_along_each_line(&[[0, 0], [1, 0]], &[[0, 1], [1, 1]]),
            None
        );
    }

    #[test]
    fn can_find_lines_meet_with_negative_x() {
        assert_eq!(
            find_units_along_each_line(&[[0, 1], [1, 2]], &[[0, 0], [1, 0]]),
            None
        );
    }

    #[test]
    fn can_find_lines_meet_with_negative_y() {
        assert_eq!(
            find_units_along_each_line(&[[0, 0], [0, 1]], &[[1, 0], [2, 1]]),
            None
        );
    }

    #[test]
    fn can_find_lines_cross_on_a_non_whole_point() {
        assert_eq!(
            find_units_along_each_line(&[[0, 0], [1, 1]], &[[0, 1], [1, 0]]),
            None
        );
    }
}
