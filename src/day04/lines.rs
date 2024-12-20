use std::cmp::min;

pub type Point = (usize, usize);
pub type Line = Vec<Point>;
pub type LinesIter = dyn Iterator<Item = Line>;

pub fn generate_lines(width: usize, height: usize) -> Box<LinesIter> {
    Box::new(
        generate_horizontals(width, height)
            .chain(generate_verticals(width, height))
            .chain(generate_diagonals(width, height)),
    )
}
fn generate_horizontals(width: usize, height: usize) -> Box<LinesIter> {
    let starts_at_left = 0..height;
    Box::new(starts_at_left.map(move |y| horizontal_from_left(width, y)))
}
fn horizontal_from_left(width: usize, y: usize) -> Line {
    let progress = 0..width;
    progress.clone().map(move |x| (x, y)).collect()
}
fn generate_verticals(width: usize, height: usize) -> Box<LinesIter> {
    let starts_at_top = 0..width;
    Box::new(starts_at_top.map(move |x| vertical_from_top(x, height)))
}
fn vertical_from_top(x: usize, height: usize) -> Line {
    let progress = 0..height;
    progress.map(move |y| (x, y)).collect()
}
fn generate_diagonals(width: usize, height: usize) -> Box<LinesIter> {
    let down_right = generate_diagonals_down_right(width, height);
    let up_right = generate_diagonals_up_right(width, height);
    Box::new(down_right.chain(up_right))
}

fn generate_diagonals_down_right(width: usize, height: usize) -> Box<LinesIter> {
    let starts_at_side = (1..(height - 1)).rev();
    let starts_at_top = 0..(width - 1);
    Box::new(
        starts_at_side
            .map(move |y| diagonal_from_side_down(width, height, y))
            .chain(starts_at_top.map(move |x| diagonal_from_top(width, height, x))),
    )
}

fn generate_diagonals_up_right(width: usize, height: usize) -> Box<LinesIter> {
    let starts_at_side = 1..(height - 1);
    let starts_at_bottom = 0..(width - 1);
    Box::new(
        starts_at_side
            .map(move |y| diagonal_from_side_up(width, y))
            .chain(starts_at_bottom.map(move |x| diagonal_from_bottom(width, height, x))),
    )
}

fn diagonal_from_top(width: usize, height: usize, x: usize) -> Line {
    let diffs = 0..min(height, width - x);
    diffs.map(move |diff| (x + diff, diff)).collect()
}

fn diagonal_from_bottom(width: usize, height: usize, x: usize) -> Line {
    let diffs = 0..min(height, width - x);
    diffs
        .map(move |diff| (x + diff, height - diff - 1))
        .collect()
}

fn diagonal_from_side_down(width: usize, height: usize, y: usize) -> Line {
    let diffs = 0..min(width, height - y);
    diffs.map(move |diff| (diff, y + diff)).collect()
}

fn diagonal_from_side_up(width: usize, y: usize) -> Line {
    let diffs = 0..min(width, y + 1);
    diffs.map(move |diff| (diff, y - diff)).collect()
}

pub fn generate_x_lines(width: usize, height: usize) -> Box<LinesIter> {
    Box::new(
        at_x_positions(width, height, generate_right_x)
            .chain(at_x_positions(width, height, generate_left_x))
            .chain(at_x_positions(width, height, generate_up_x))
            .chain(at_x_positions(width, height, generate_down_x)),
    )
}

fn at_x_positions(
    width: usize,
    height: usize,
    generator: fn(x: usize, y: usize) -> Line,
) -> Box<LinesIter> {
    Box::new((0..=(width - 3)).flat_map(move |x| (0..=(height - 3)).map(move |y| generator(x, y))))
}

fn generate_right_x(x: usize, y: usize) -> Line {
    (0..3)
        .map(|n| (n + x, n + y))
        .chain((0..3).map(|n| (n + x, 2 - n + y)))
        .collect()
}

fn generate_left_x(x: usize, y: usize) -> Line {
    (0..3)
        .map(|n| (2 - n + x, n + y))
        .chain((0..3).map(|n| (2 - n + x, 2 - n + y)))
        .collect()
}

fn generate_up_x(x: usize, y: usize) -> Line {
    (0..3)
        .map(|n| (n + x, 2 - n + y))
        .chain((0..3).map(|n| (2 - n + x, 2 - n + y)))
        .collect()
}

fn generate_down_x(x: usize, y: usize) -> Line {
    (0..3)
        .map(|n| (n + x, n + y))
        .chain((0..3).map(|n| (2 - n + x, n + y)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use std::collections::HashMap;
    use std::fmt::Write;

    #[test]
    fn can_generate_2_by_2_lines() {
        assert_snapshot!(print_lines(2, 2))
    }

    #[test]
    fn can_generate_3_by_3_lines() {
        assert_snapshot!(print_lines(3, 3))
    }

    #[test]
    fn can_generate_4_by_4_lines() {
        assert_snapshot!(print_lines(4, 4))
    }

    #[test]
    fn can_generate_10_by_10_lines() {
        assert_snapshot!(print_lines(10, 10))
    }

    #[test]
    fn can_generate_x_lines() {
        assert_snapshot!(print_x_lines(4, 4))
    }

    fn print_lines(width: usize, height: usize) -> String {
        print_lines_iter(generate_lines(width, height), width, height)
    }

    fn print_x_lines(width: usize, height: usize) -> String {
        print_lines_iter(generate_x_lines(width, height), width, height)
    }

    fn print_lines_iter(lines: Box<LinesIter>, width: usize, height: usize) -> String {
        let mut str: String = "".to_owned();
        write_lines_iter(lines, width, height, &mut str).unwrap();
        str
    }

    fn write_lines_iter<W: Write>(
        lines: Box<LinesIter>,
        width: usize,
        height: usize,
        out: &mut W,
    ) -> std::fmt::Result {
        for line in lines {
            write_line(line, width, height, out)?;
            out.write_char('\n')?
        }
        Ok(())
    }

    fn write_line<W: Write>(
        line: Line,
        width: usize,
        height: usize,
        out: &mut W,
    ) -> std::fmt::Result {
        let point_to_n: HashMap<(usize, usize), usize> =
            HashMap::from_iter(line.iter().enumerate().map(|(n, point)| (*point, n)));
        for y in 0..height {
            for x in 0..width {
                if let Some(n) = point_to_n.get(&(x, y)) {
                    out.write_char(char::from_digit(((n + 1) % 10) as u32, 10).unwrap())?;
                } else {
                    out.write_char('.')?;
                }
            }
            out.write_char('\n')?;
        }
        Ok(())
    }
}
