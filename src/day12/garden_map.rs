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

    pub(crate) fn inc_edge(&self, edge: &Edge, dim_index: usize) -> Option<Edge> {
        self.acc_edge(edge, dim_index, 1)
    }

    pub(crate) fn dec_edge(&self, edge: &Edge, dim_index: usize) -> Option<Edge> {
        self.acc_edge(edge, dim_index, -1)
    }

    fn acc_edge(&self, edge: &Edge, dim_index: usize, inc: isize) -> Option<Edge> {
        let limit = if dim_index == 0 {
            self.width
        } else {
            self.height
        };
        let new_value = edge[0][dim_index].wrapping_add_signed(inc);
        let mut new_edge = *edge;
        new_edge[0][dim_index] = new_value;
        new_edge[1][dim_index] = new_value;
        if new_value > limit && new_value < usize::MAX {
            None
        } else {
            Some(new_edge)
        }
    }
}

pub(crate) type Point = [usize; 2];

pub(crate) type Edge = [Point; 2];

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
