use crate::day9::file_system::FileSystem;
use std::str::FromStr;

pub struct DiskMap {
    length: usize,
    files: Vec<File>,
    spaces: Vec<Space>,
}

struct Space {
    index: usize,
    length: usize,
}

struct File {
    id: usize,
    index: usize,
    length: usize,
}

impl DiskMap {
    pub fn parse(string: &str) -> DiskMap {
        let sizes: Vec<usize> = string
            .chars()
            .flat_map(|char| usize::from_str(char.to_string().as_str()))
            .collect();
        let mut files: Vec<File> = Vec::with_capacity(sizes.len() / 2 + 1);
        let mut spaces: Vec<Space> = Vec::with_capacity(sizes.len() / 2);
        let mut pos = 0;
        for i in (0..sizes.len()).step_by(2) {
            let file_id = i / 2;
            let file_size = sizes[i];
            files.push(File {
                id: file_id,
                index: pos,
                length: file_size,
            });
            pos += file_size;
            if let Some(space_size) = sizes.get(i + 1) {
                spaces.push(Space {
                    index: pos,
                    length: *space_size,
                });
                pos += space_size;
            }
        }
        DiskMap {
            length: pos,
            files,
            spaces,
        }
    }
    pub fn build_file_system(&self) -> FileSystem {
        let mut fs = FileSystem::with_capacity(self.length);
        for file in self.files.iter() {
            fs.add_file(file.id, file.index, file.length);
        }
        fs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day9::file_system::tests as fs;

    #[test]
    fn can_compact_fitting_into_spaces() {
        let map = DiskMap::parse("14213");
        let fs = map.build_file_system();
        assert_eq!(fs::print(&fs), "0....11.222")
    }
}
