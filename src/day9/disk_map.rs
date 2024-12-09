use crate::day9::file_system::FileSystem;
use std::str::FromStr;

pub struct DiskMap {
    sizes: Vec<usize>,
}

impl DiskMap {
    pub fn parse(string: &str) -> DiskMap {
        let sizes = string
            .chars()
            .flat_map(|char| usize::from_str(char.to_string().as_str()))
            .collect();
        DiskMap { sizes }
    }
    pub fn build_file_system(&self) -> FileSystem {
        let mut fs = FileSystem::with_capacity(self.sizes.iter().sum());
        for i in (0..self.sizes.len()).step_by(2) {
            let file_id = i / 2;
            let file_size = self.sizes[i];
            let space_size = *self.sizes.get(i + 1).unwrap_or(&0);
            fs.add_file(file_id, file_size);
            fs.add_space(space_size);
        }
        fs
    }
}
