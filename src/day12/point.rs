pub type Point = [usize; 2];

pub fn adjacent_points(point: Point) -> Vec<Point> {
    let [x, y] = point;
    vec![
        [x + 1, y],
        [x, y + 1],
        [x.wrapping_sub(1), y],
        [x, y.wrapping_sub(1)],
    ]
}
