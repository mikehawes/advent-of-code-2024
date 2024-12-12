use crate::day12::region::build_regions;

pub struct GardenMap {
    plots: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl GardenMap {
    pub fn parse(string: &str) -> GardenMap {
        let mut plots = vec![];
        for line in string.lines() {
            let mut line_plots = Vec::with_capacity(line.len());
            for c in line.chars() {
                line_plots.push(c);
            }
            plots.push(line_plots);
        }
        let width = plots.first().map(|line| line.len()).unwrap_or(0);
        let height = plots.len();
        GardenMap {
            plots,
            width,
            height,
        }
    }
    pub fn sum_fencing_price(&self) -> usize {
        build_regions(self)
            .iter()
            .map(|region| region.fencing_price())
            .sum()
    }
    pub(crate) fn plant_at(&self, point: Point) -> char {
        let (x, y) = point;
        self.plots[y][x]
    }
    pub(crate) fn is_on_map(&self, point: Point) -> bool {
        let (x, y) = point;
        x < self.width && y < self.height
    }
    pub(crate) fn points(&self) -> impl Iterator<Item = Point> + use<'_> {
        (0..self.height).flat_map(|y| (0..self.width).map(move |x| (x, y)))
    }
    pub(crate) fn sub_x_for_edge(&self, x: usize, sub: usize) -> usize {
        self.overflow_x(x.wrapping_sub(sub))
    }
    pub(crate) fn add_x_for_edge(&self, x: usize, add: usize) -> usize {
        self.overflow_x(x.wrapping_add(add))
    }
    pub(crate) fn sub_y_for_edge(&self, y: usize, sub: usize) -> usize {
        self.overflow_y(y.wrapping_sub(sub))
    }
    pub(crate) fn add_y_for_edge(&self, y: usize, add: usize) -> usize {
        self.overflow_y(y.wrapping_add(add))
    }
    fn overflow_x(&self, x: usize) -> usize {
        if x > self.width {
            usize::MAX
        } else {
            x
        }
    }
    fn overflow_y(&self, y: usize) -> usize {
        if y > self.height {
            usize::MAX
        } else {
            y
        }
    }
}

pub(crate) type Point = (usize, usize);

#[cfg(test)]
mod tests {
    use crate::day12::garden_map::GardenMap;

    #[test]
    fn can_parse_map() {
        let string = "\
            AAAA\n\
            BBCD\n\
            BBCC\n\
            EEEC\n";
        let map = GardenMap::parse(string);
        assert_eq!(print(&map), string)
    }

    #[test]
    fn can_price_fencing_for_first_example() {
        let string = "\
            AAAA\n\
            BBCD\n\
            BBCC\n\
            EEEC\n";
        let map = GardenMap::parse(string);
        assert_eq!(map.sum_fencing_price(), 140)
    }

    fn print(map: &GardenMap) -> String {
        let mut str = String::new();
        for line in map.plots.iter() {
            str.push_str(&print_line(line));
            str.push('\n');
        }
        str
    }
    fn print_line(line: &[char]) -> String {
        line.iter().collect()
    }
}
