use std::cmp::min;

#[derive(Clone)]
pub struct FileSystem {
    file_ids: Vec<isize>,
}

impl FileSystem {
    pub fn from(file_ids: Vec<isize>) -> FileSystem {
        FileSystem { file_ids }
    }
    pub fn compact_last_first(&self) -> FileSystem {
        let mut file_ids = self.file_ids.clone();
        let mut ids_from_end = self
            .file_ids
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, id)| **id != -1);
        let mut new_len = self.file_ids.len();
        for (index, id) in file_ids.iter_mut().enumerate() {
            if *id != -1 {
                continue;
            }
            let (new_id_index, new_id) = ids_from_end.next().unwrap();
            if new_id_index < index {
                new_len = min(new_len, index);
                break;
            }
            *id = *new_id;
            new_len = new_id_index;
        }
        file_ids.truncate(new_len);
        FileSystem { file_ids }
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

    #[test]
    fn can_compact_last_first() {
        let fs = DiskMap::parse("12345").build_file_system();
        assert_eq!(print(&fs.compact_last_first()), "022111222")
    }

    #[test]
    fn can_build_example_file_system() {
        let string = "2333133121414131402";
        let fs = DiskMap::parse(string).build_file_system();
        assert_eq!(print(&fs), "00...111...2...333.44.5555.6666.777.888899");
    }

    #[test]
    fn can_compact_example_file_system() {
        let string = "2333133121414131402";
        let fs = DiskMap::parse(string).build_file_system();
        assert_eq!(
            print(&fs.compact_last_first()),
            "0099811188827773336446555566"
        );
    }

    #[test]
    fn can_compute_example_checksum() {
        let string = "2333133121414131402";
        let fs = DiskMap::parse(string).build_file_system();
        assert_eq!(fs.compact_last_first().checksum(), 1928);
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