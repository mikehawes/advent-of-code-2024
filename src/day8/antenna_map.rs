pub struct AntennaMap {
    width: usize,
    height: usize,
}

impl AntennaMap {
    pub fn parse(string: &str) -> AntennaMap {
        let width = string.lines().next().unwrap().len();
        let height = string.lines().count();
        AntennaMap { width, height }
    }
    pub fn count_unique_antinode_locations(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::day8::antenna_map::AntennaMap;

    #[test]
    fn can_plot_map() {
        let input = "\
                ...\n\
                ...\n";
        assert_eq!(
            plot_map(&AntennaMap::parse(input)),
            "...\n\
             ...\n"
                .to_string()
        );
    }

    fn plot_map(map: &AntennaMap) -> String {
        let mut str = "".to_string();
        for _ in 0..map.height {
            for _ in 0..map.width {
                str.push('.');
            }
            str.push('\n');
        }
        str
    }
}
