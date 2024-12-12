use crate::day12::garden_map::GardenMap;
use crate::day12::point::Point;
use std::collections::HashSet;

pub type Edge = [Point; 2];

pub fn along_dim_index(edge: &Edge) -> usize {
    let [[x1, _], [x2, _]] = edge;
    if x1 == x2 {
        0
    } else {
        1
    }
}

pub fn edge(a: Point, b: Point) -> Edge {
    let mut points = [a, b];
    points.sort();
    points
}

pub fn adjacent_edges(from: &Edge, region_edges: &HashSet<Edge>, map: &GardenMap) -> Vec<Edge> {
    let mut edges = Vec::with_capacity(2);
    for direction in [-1, 1] {
        if map
            .obstructing_edges(from, direction)
            .iter()
            .any(|obstruction| region_edges.contains(obstruction))
        {
            continue;
        }
        if let Some(adjacent) = map.adjacent_edge(from, direction) {
            edges.push(adjacent);
        }
    }
    edges
}
