use crate::day12::garden_map::{GardenMap, Point};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct Region {
    number: usize,
    plant: char,
    area: usize,
    perimeter: usize,
}

impl Region {
    pub fn fencing_price(&self) -> usize {
        self.area * self.perimeter
    }
}

pub fn build_regions(map: &GardenMap) -> Vec<Region> {
    let mut point_to_region_number = HashMap::new();
    build_regions_and_points(map, &mut point_to_region_number)
}

fn build_regions_and_points(
    map: &GardenMap,
    point_to_region_number: &mut HashMap<Point, usize>,
) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    let mut region_number = 0;
    for point in map.points() {
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
        area: 0,
        perimeter: 0,
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
    region.area += 1;
    for adjacent in adjacent_points(point) {
        if !map.is_on_map(adjacent) {
            region.perimeter += 1;
            continue;
        }
        let adjacent_plant = map.plant_at(adjacent);
        if region.plant == adjacent_plant {
            if point_to_region_number.contains_key(&adjacent) {
                continue;
            }
            map_region(adjacent, region, map, point_to_region_number);
        } else {
            region.perimeter += 1;
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
    use super::*;
    use std::cmp::max;

    #[test]
    fn can_number_regions_for_first_example() {
        // Given
        let string = "\
            AAAA\n\
            BBCD\n\
            BBCC\n\
            EEEC\n";
        let map = GardenMap::parse(string);

        // When
        let mut point_to_region_number = HashMap::new();
        build_regions_and_points(&map, &mut point_to_region_number);

        // Then
        let expected = "\
            0000\n\
            1123\n\
            1122\n\
            4442\n";
        assert_eq!(print_region_numbers(&point_to_region_number), expected)
    }

    #[test]
    fn can_get_region_details_for_first_example() {
        let string = "\
            AAAA\n\
            BBCD\n\
            BBCC\n\
            EEEC\n";
        let map = GardenMap::parse(string);

        assert_eq!(
            build_regions(&map),
            vec![
                Region {
                    number: 0,
                    plant: 'A',
                    area: 4,
                    perimeter: 10
                },
                Region {
                    number: 1,
                    plant: 'B',
                    area: 4,
                    perimeter: 8
                },
                Region {
                    number: 2,
                    plant: 'C',
                    area: 4,
                    perimeter: 10
                },
                Region {
                    number: 3,
                    plant: 'D',
                    area: 1,
                    perimeter: 4
                },
                Region {
                    number: 4,
                    plant: 'E',
                    area: 3,
                    perimeter: 8
                }
            ]
        )
    }

    fn print_region_numbers(point_to_region_number: &HashMap<Point, usize>) -> String {
        let mut width = 0;
        let mut height = 0;
        for (x, y) in point_to_region_number.keys() {
            width = max(width, *x);
            height = max(height, *y);
        }
        let mut str = String::new();
        for y in 0..=height {
            for x in 0..=width {
                let region_number = point_to_region_number[&(x, y)];
                str.push_str(&region_number.to_string());
            }
            str.push('\n');
        }
        str
    }
}
