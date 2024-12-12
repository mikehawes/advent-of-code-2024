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
        build_regions(self)
            .iter()
            .map(|region| region.edges * region.plots)
            .sum()
    }
    fn plant_at(&self, point: Point) -> char {
        let (x, y) = point;
        self.plots[y][x]
    }
    fn is_on_map(&self, point: Point) -> bool {
        let (x, y) = point;
        x < self.width && y < self.height
    }
}

type Point = (usize, usize);

struct Region {
    number: usize,
    plant: char,
    plots: usize,
    edges: usize,
}

fn build_regions(map: &GardenMap) -> Vec<Region> {
    let mut point_to_region_number = HashMap::new();
    build_regions_and_points(map, &mut point_to_region_number)
}

fn build_regions_and_points(
    map: &GardenMap,
    point_to_region_number: &mut HashMap<Point, usize>,
) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    let mut region_number = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let point = (x, y);
            if !point_to_region_number.contains_key(&point) {
                regions.push(map_region_from(
                    point,
                    region_number,
                    map,
                    point_to_region_number,
                ));
                region_number += 1;
            }
        }
    }
    regions
}

fn map_region_from(
    start_point: Point,
    number: usize,
    map: &GardenMap,
    point_to_region_number: &mut HashMap<Point, usize>,
) -> Region {
    let plant = map.plant_at(start_point);
    let mut region = Region {
        number,
        plant,
        plots: 0,
        edges: 0,
    };
    map_region(start_point, &mut region, map, point_to_region_number);
    region
}

fn map_region(
    point: Point,
    region: &mut Region,
    map: &GardenMap,
    point_to_region_number: &mut HashMap<Point, usize>,
) {
    point_to_region_number.insert(point, region.number);
    region.plots += 1;
    for adjacent in adjacent_points(point) {
        if !map.is_on_map(adjacent) {
            region.edges += 1;
            continue;
        }
        let adjacent_plant = map.plant_at(adjacent);
        if region.plant == adjacent_plant {
            if point_to_region_number.contains_key(&adjacent) {
                continue;
            }
            map_region(adjacent, region, map, point_to_region_number);
        } else {
            region.edges += 1;
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
    use crate::day12::garden_map::{build_regions_and_points, GardenMap, Point};
    use std::collections::HashMap;

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
    fn can_number_regions_for_first_example() {
        let string = "\
            AAAA\n\
            BBCD\n\
            BBCC\n\
            EEEC\n";
        let map = GardenMap::parse(string);
        let mut point_to_region_number = HashMap::new();
        build_regions_and_points(&map, &mut point_to_region_number);
        let expected = "\
            0000\n\
            1123\n\
            1122\n\
            4442\n";
        assert_eq!(
            print_region_numbers(&map, &point_to_region_number),
            expected
        )
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

    fn print_region_numbers(
        map: &GardenMap,
        point_to_region_number: &HashMap<Point, usize>,
    ) -> String {
        let mut str = String::new();
        for y in 0..map.height {
            for x in 0..map.width {
                let region_number = point_to_region_number[&(x, y)];
                str.push_str(&region_number.to_string());
            }
            str.push('\n');
        }
        str
    }
}
