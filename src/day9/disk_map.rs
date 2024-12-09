use crate::day9::file_system::FileSystem;
use std::str::FromStr;

pub struct DiskMap {
    sizes: Vec<usize>,
}

impl DiskMap {
    pub fn parse(string: &str) -> DiskMap {
        let sizes = string
            .chars()
            .map(|char| usize::from_str(char.to_string().as_str()).unwrap())
            .collect();
        DiskMap { sizes }
    }
    pub fn build_file_system(&self) -> FileSystem {
        let file_ids =
            (0..self.sizes.len())
                .step_by(2)
                .flat_map(|i| {
                    let file_id = i / 2;
                    let file_size = self.sizes[i];
                    let space_size = *self.sizes.get(i + 1).unwrap_or(&0);
                    (0..(file_size + space_size)).map(move |j| {
                        if j < file_size {
                            file_id as isize
                        } else {
                            -1
                        }
                    })
                })
                .collect();
        FileSystem::from(file_ids)
    }
}
