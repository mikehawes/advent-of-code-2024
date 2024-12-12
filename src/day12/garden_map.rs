use std::collections::HashMap;

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
    fn plant_at(&self, point: Point) -> char {
        let (x, y) = point;
        self.plots[y][x]
    }
    fn adjacent_points(&self, point: Point) -> Vec<Point> {
        adjacent_points(point)
            .iter()
            .filter(|p| self.is_on_map(**p))
            .copied()
            .collect()
    }
    fn is_on_map(&self, point: Point) -> bool {
        let (x, y) = point;
        x < self.width && y < self.height
    }
}

struct RegionIndex<'a> {
    map: &'a GardenMap,
    plot_region_numbers: Vec<Vec<usize>>,
}

type Point = (usize, usize);

impl<'a> RegionIndex<'a> {
    fn from(map: &GardenMap) -> RegionIndex {
        let mut point_to_region_number: HashMap<Point, usize> = HashMap::new();
        let mut region_number = 0;
        for y in 0..map.height {
            for x in 0..map.width {
                let point = (x, y);
                if !point_to_region_number.contains_key(&point) {
                    map_region_from(point, region_number, map, &mut point_to_region_number);
                    region_number += 1;
                }
            }
        }
        let plot_region_numbers = Vec::from_iter(
            (0..map.height)
                .map(|y| Vec::from_iter((0..map.width).map(|x| point_to_region_number[&(x, y)]))),
        );
        RegionIndex {
            map,
            plot_region_numbers,
        }
    }
    fn region_number_at(&self, point: Point) -> usize {
        let (x, y) = point;
        self.plot_region_numbers[y][x]
    }
}

fn map_region_from(
    start_point: Point,
    region_number: usize,
    map: &GardenMap,
    point_to_region_number: &mut HashMap<Point, usize>,
) {
    let plant = map.plant_at(start_point);
    map_region(
        start_point,
        region_number,
        plant,
        map,
        point_to_region_number,
    )
}

fn map_region(
    point: Point,
    region_number: usize,
    plant: char,
    map: &GardenMap,
    point_to_region_number: &mut HashMap<Point, usize>,
) {
    point_to_region_number.insert(point, region_number);
    for adjacent in map.adjacent_points(point) {
        if point_to_region_number.contains_key(&adjacent) {
            continue;
        }
        let adjacent_plant = map.plant_at(adjacent);
        if plant == adjacent_plant {
            map_region(adjacent, region_number, plant, map, point_to_region_number);
        }
    }
}

fn adjacent_points(point: Point) -> Vec<Point> {
    let (x, y) = point;
    vec![
        (x + 1, y),
        (x, y + 1),
        (x.wrapping_sub(1), y),
        (x, y.wrapping_sub(1)),
    ]
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
            1123\n\
            1122\n\
            4442\n";
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
