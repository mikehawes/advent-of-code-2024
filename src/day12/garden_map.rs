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
