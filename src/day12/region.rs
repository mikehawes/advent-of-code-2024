use crate::day12::garden_map::{GardenMap, Point};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq)]
pub struct Region {
    number: usize,
    plant: char,
    area: usize,
    perimeter: usize,
    sides: usize,
}

impl Region {
    pub fn fencing_price(&self) -> usize {
        self.area * self.perimeter
    }
}

type Edge = (Point, Point);

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
        sides: 0,
    };
    let mut edges = HashSet::new();
    map_region(
        start_point,
        &mut region,
        map,
        point_to_region_number,
        &mut edges,
    );
    region.perimeter = edges.len();
    region.sides = count_sides(&edges, map);
    region
}

fn map_region(
    point: Point,
    region: &mut Region,
    map: &GardenMap,
    point_to_region_number: &mut HashMap<Point, usize>,
    edges: &mut HashSet<Edge>,
) {
    point_to_region_number.insert(point, region.number);
    region.area += 1;
    for adjacent in adjacent_points(point, map) {
        if !map.is_on_map(adjacent) {
            edges.insert(edge(point, adjacent));
            continue;
        }
        let adjacent_plant = map.plant_at(adjacent);
        if region.plant == adjacent_plant {
            if point_to_region_number.contains_key(&adjacent) {
                continue;
            }
            map_region(adjacent, region, map, point_to_region_number, edges);
        } else {
            edges.insert(edge(point, adjacent));
        }
    }
}

fn count_sides(edges: &HashSet<Edge>, map: &GardenMap) -> usize {
    let mut remaining = edges.clone();
    let mut count = 0;
    for edge in edges {
        if remove_side_edges(edge, map, &mut remaining) {
            count += 1;
        }
    }
    count
}

fn remove_side_edges(edge: &Edge, map: &GardenMap, remaining: &mut HashSet<Edge>) -> bool {
    if !remaining.remove(edge) {
        return false;
    }
    for adjacent in adjacent_edges(edge, map) {
        remove_side_edges(&adjacent, map, remaining);
    }
    true
}

fn adjacent_points(point: Point, map: &GardenMap) -> Vec<Point> {
    let (x, y) = point;
    vec![
        (map.add_x_for_edge(x, 1), y),
        (x, map.add_y_for_edge(y, 1)),
        (map.sub_x_for_edge(x, 1), y),
        (x, map.sub_y_for_edge(y, 1)),
    ]
}

fn edge(a: Point, b: Point) -> Edge {
    let mut points = [a, b];
    points.sort();
    (points[0], points[1])
}

fn adjacent_edges(from: &Edge, map: &GardenMap) -> Vec<Edge> {
    let ((x1, y1), (x2, y2)) = from;
    if x1 == x2 {
        let x = *x1;
        vec![
            edge((x, map.sub_y_for_edge(*y1, 1)), (x, *y1)),
            edge((x, *y2), (x, map.add_y_for_edge(*y2, 1))),
        ]
    } else {
        let y = *y1;
        vec![
            edge((map.sub_x_for_edge(*x1, 1), y), (*x1, y)),
            edge((*x2, y), (map.add_x_for_edge(*x2, 1), y)),
        ]
    }
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
    fn can_get_region_areas_for_first_example() {
        let string = "\
            AAAA\n\
            BBCD\n\
            BBCC\n\
            EEEC\n";
        let map = GardenMap::parse(string);

        assert_eq!(
            build_regions(&map)
                .iter()
                .map(|region| (region.plant, region.area))
                .collect::<Vec<(char, usize)>>(),
            vec![('A', 4), ('B', 4), ('C', 4), ('D', 1), ('E', 3)]
        )
    }

    #[test]
    fn can_get_region_perimeters_for_first_example() {
        let string = "\
            AAAA\n\
            BBCD\n\
            BBCC\n\
            EEEC\n";
        let map = GardenMap::parse(string);

        assert_eq!(
            build_regions(&map)
                .iter()
                .map(|region| (region.plant, region.perimeter))
                .collect::<Vec<(char, usize)>>(),
            vec![('A', 10), ('B', 8), ('C', 10), ('D', 4), ('E', 8)]
        )
    }

    #[test]
    #[ignore]
    fn can_get_region_sides_for_first_example() {
        let string = "\
            AAAA\n\
            BBCD\n\
            BBCC\n\
            EEEC\n";
        let map = GardenMap::parse(string);

        assert_eq!(
            build_regions(&map)
                .iter()
                .map(|region| (region.plant, region.sides))
                .collect::<Vec<(char, usize)>>(),
            vec![('A', 4), ('B', 4), ('C', 8), ('D', 4), ('E', 4)]
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
