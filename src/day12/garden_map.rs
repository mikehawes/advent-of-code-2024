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
    pub fn sum_fencing_price_bulk_discount(&self) -> usize {
        build_regions(self)
            .iter()
            .map(|region| region.fencing_price_bulk_discount())
            .sum()
    }
    pub(crate) fn plant_at(&self, point: Point) -> char {
        let [x, y] = point;
        self.plots[y][x]
    }
    pub(crate) fn is_on_map(&self, point: Point) -> bool {
        let [x, y] = point;
        x < self.width && y < self.height
    }
    pub(crate) fn points(&self) -> impl Iterator<Item = Point> + use<'_> {
        (0..self.height).flat_map(|y| (0..self.width).map(move |x| [x, y]))
    }
    pub(crate) fn adjacent_edge(&self, from: &Edge, direction: isize) -> Option<Edge> {
        let along_dim = along_dim_index(from);
        let new_value = from[0][along_dim].wrapping_add_signed(direction);
        let mut new_edge = *from;
        new_edge[0][along_dim] = new_value;
        new_edge[1][along_dim] = new_value;
        if self.is_outside_limits(along_dim, new_value) {
            None
        } else {
            Some(new_edge)
        }
    }
    pub(crate) fn obstructing_edges(&self, from: &Edge, direction: isize) -> Vec<Edge> {
        let along_dim = along_dim_index(from);
        let mut end_0 = from[0];
        end_0[along_dim] = end_0[along_dim].wrapping_add_signed(direction);
        let mut end_1 = from[1];
        end_1[along_dim] = end_1[along_dim].wrapping_add_signed(direction);
        vec![edge(from[0], end_0), edge(from[1], end_1)]
    }
    fn is_outside_limits(&self, dim_index: usize, value: usize) -> bool {
        value > self.dim_index_limit(dim_index) && value < usize::MAX
    }
    fn dim_index_limit(&self, dim_index: usize) -> usize {
        if dim_index == 0 {
            self.width
        } else {
            self.height
        }
    }
}

pub(crate) type Point = [usize; 2];

pub(crate) type Edge = [Point; 2];

fn along_dim_index(edge: &Edge) -> usize {
    let [[x1, _], [x2, _]] = edge;
    if x1 == x2 {
        0
    } else {
        1
    }
}

pub(crate) fn edge(a: Point, b: Point) -> Edge {
    let mut points = [a, b];
    points.sort();
    points
}

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

    #[test]
    fn can_bulk_price_fencing_for_first_example() {
        let string = "\
            AAAA\n\
            BBCD\n\
            BBCC\n\
            EEEC\n";
        let map = GardenMap::parse(string);
        assert_eq!(map.sum_fencing_price_bulk_discount(), 80)
    }

    #[test]
    fn can_bulk_price_fencing_for_e() {
        let string = "\
            EEEEE\n\
            EXXXX\n\
            EEEEE\n\
            EXXXX\n\
            EEEEE\n";
        let map = GardenMap::parse(string);
        assert_eq!(map.sum_fencing_price_bulk_discount(), 236)
    }

    #[test]
    fn can_bulk_price_fencing_when_touching_diagonally() {
        let string = "\
            AAAAAA\n\
            AAABBA\n\
            AAABBA\n\
            ABBAAA\n\
            ABBAAA\n\
            AAAAAA\n";
        let map = GardenMap::parse(string);
        assert_eq!(map.sum_fencing_price_bulk_discount(), 368)
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
