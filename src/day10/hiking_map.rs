use std::collections::HashSet;

pub struct HikingMap {
    tiles: Vec<Vec<usize>>,
    trailheads: Vec<Point>,
    width: usize,
    height: usize,
}

type Point = (usize, usize);

impl HikingMap {
    pub fn parse(string: &str) -> HikingMap {
        let mut tiles = Vec::new();
        let mut trailheads = Vec::new();
        for (y, line) in string.lines().enumerate() {
            let mut line_tiles = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let tile = c.to_digit(10).unwrap_or(10) as usize;
                line_tiles.push(tile);
                if tile == 0 {
                    trailheads.push((x, y));
                }
            }
            tiles.push(line_tiles);
        }
        let width = tiles.first().map(|line| line.len()).unwrap_or(0);
        let height = tiles.len();
        HikingMap {
            tiles,
            trailheads,
            width,
            height,
        }
    }
    pub fn sum_trailhead_scores(&self) -> usize {
        self.trailhead_scores().map(|(_, score)| score).sum()
    }
    pub fn sum_trailhead_ratings(&self) -> usize {
        self.trailhead_ratings().map(|(_, score)| score).sum()
    }
    fn trailhead_scores(&self) -> impl Iterator<Item = (Point, usize)> + use<'_> {
        self.trailheads
            .iter()
            .map(|trailhead| (*trailhead, self.reachable_9s(*trailhead).len()))
    }
    fn trailhead_ratings(&self) -> impl Iterator<Item = (Point, usize)> + use<'_> {
        self.trailheads
            .iter()
            .map(|trailhead| (*trailhead, self.reachable_paths(*trailhead)))
    }
    fn reachable_9s(&self, point: Point) -> HashSet<Point> {
        let mut set = HashSet::new();
        self.for_each_reached_9(point, &mut |point| {
            set.insert(point);
        });
        set
    }
    fn reachable_paths(&self, point: Point) -> usize {
        let mut paths = 0;
        self.for_each_reached_9(point, &mut |_| {
            paths += 1;
        });
        paths
    }
    fn for_each_reached_9<F>(&self, point: Point, operation: &mut F)
    where
        F: FnMut(Point),
    {
        let height = self.height_at(point);
        if height == 9 {
            operation(point)
        } else {
            let next_height = height + 1;
            self.adjacent_points(point)
                .iter()
                .filter(|adj| self.height_at(**adj) == next_height)
                .for_each(|adj| self.for_each_reached_9(*adj, operation))
        }
    }
    fn height_at(&self, point: Point) -> usize {
        let (x, y) = point;
        self.tiles[y][x]
    }
    fn adjacent_points(&self, point: Point) -> Vec<Point> {
        let (x, y) = point;
        let mut points = vec![];
        for (x1, y1) in [
            (x + 1, y),
            (x, y + 1),
            (x.wrapping_sub(1), y),
            (x, y.wrapping_sub(1)),
        ] {
            if x1 < self.width && y1 < self.height {
                points.push((x1, y1));
            }
        }
        points
    }
}

#[cfg(test)]
mod tests {
    use crate::day10::hiking_map::{HikingMap, Point};
    use crate::input::input_to_string;

    #[test]
    fn can_find_score_0_trailhead() {
        let map = HikingMap::parse("012");
        assert_eq!(map.sum_trailhead_scores(), 0)
    }

    #[test]
    fn can_find_score_1_trailhead() {
        let map = HikingMap::parse("0123456789");
        assert_eq!(map.sum_trailhead_scores(), 1)
    }

    #[test]
    fn can_find_score_2_trailhead() {
        let map = HikingMap::parse("9876543210123456789");
        assert_eq!(map.sum_trailhead_scores(), 2)
    }

    #[test]
    fn can_find_example_trailhead_scores() {
        let string = input_to_string("day10/example.txt").unwrap();
        let map = HikingMap::parse(string.as_str());
        let scores: Vec<(Point, usize)> = map.trailhead_scores().collect();
        assert_eq!(
            scores,
            vec![
                ((2, 0), 5),
                ((4, 0), 6),
                ((4, 2), 5),
                ((6, 4), 3),
                ((2, 5), 1),
                ((5, 5), 3),
                ((0, 6), 5),
                ((6, 6), 3),
                ((1, 7), 5)
            ]
        )
    }

    #[test]
    fn can_sum_example_trailhead_scores() {
        let string = input_to_string("day10/example.txt").unwrap();
        let map = HikingMap::parse(string.as_str());
        assert_eq!(map.sum_trailhead_scores(), 36)
    }

    #[test]
    fn can_sum_example_trailhead_ratings() {
        let string = input_to_string("day10/example.txt").unwrap();
        let map = HikingMap::parse(string.as_str());
        assert_eq!(map.sum_trailhead_ratings(), 81)
    }
}
