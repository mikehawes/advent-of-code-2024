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
        0
    }
}

struct RegionIndex<'a> {
    map: &'a GardenMap,
    regions: Vec<Region>,
    plot_region_numbers: Vec<Vec<usize>>,
}

type Point = (usize, usize);

impl<'a> RegionIndex<'a> {
    fn from(map: &GardenMap) -> RegionIndex {
        let regions = Vec::new();
        let plot_region_numbers =
            Vec::from_iter((0..map.height).map(|_| Vec::from_iter(0..map.width)));
        RegionIndex {
            map,
            regions,
            plot_region_numbers,
        }
    }
    fn region_number_at(&self, _: Point) -> usize {
        0
    }
}

struct Region {
    number: usize,
    plant: char,
}

#[cfg(test)]
mod tests {
    use crate::day12::garden_map::{GardenMap, RegionIndex};

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
    fn can_number_regions() {
        let string = "\
            AAAA\n\
            BBCD\n\
            BBCC\n\
            EEEC\n";
        let map = GardenMap::parse(string);
        let region_index = RegionIndex::from(&map);
        let expected = "\
            0000\n\
            0000\n\
            0000\n\
            0000\n";
        assert_eq!(print_region_numbers(&region_index), expected)
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

    fn print_region_numbers(region_index: &RegionIndex) -> String {
        let mut str = String::new();
        for y in 0..region_index.map.height {
            for x in 0..region_index.map.width {
                let region_number = region_index.region_number_at((x, y));
                str.push_str(&region_number.to_string());
            }
            str.push('\n');
        }
        str
    }
}
