#[derive(Clone)]
pub struct FileSystem {
    file_ids: Vec<isize>,
}

impl FileSystem {
    pub fn from(file_ids: Vec<isize>) -> FileSystem {
        FileSystem { file_ids }
    }
    pub fn compact_last_first(&self) -> FileSystem {
        self.clone()
    }
    pub fn checksum(&self) -> usize {
        self.file_ids
            .iter()
            .enumerate()
            .map(|(index, id)| if *id < 0 { 0 } else { index * *id as usize })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::day9::disk_map::DiskMap;
    use crate::day9::file_system::FileSystem;

    #[test]
    fn can_build_file_system() {
        let map = DiskMap::parse("12345");
        assert_eq!(print(&map.build_file_system()), "0..111....22222");
    }

    #[test]
    fn can_compute_checksum() {
        let fs = DiskMap::parse("12345").build_file_system();
        assert_eq!(fs.checksum(), 3 + 4 + 5 + 2 * (10 + 11 + 12 + 13 + 14))
    }

    fn print(file_system: &FileSystem) -> String {
        file_system
            .file_ids
            .iter()
            .map(|id| match id {
                -1 => '.'.to_string(),
                id => id.to_string(),
            })
            .collect()
    }
}
