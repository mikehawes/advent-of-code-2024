use crate::day9::file_system::FileSystem;
use std::str::FromStr;

pub struct DiskMap {
    length: usize,
    files: Vec<File>,
    spaces: Vec<Space>,
}

#[derive(Clone)]
struct Space {
    index: usize,
    length: usize,
}

#[derive(Clone)]
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
    pub fn compact_fitting_into_spaces(&self) -> DiskMap {
        let mut spaces = self.spaces.clone();
        let mut files = self.files.clone();
        for file in files.iter_mut().rev() {
            for space in spaces.iter_mut() {
                if space.index < file.index && file.length <= space.length {
                    file.index = space.index;
                    space.index += file.length;
                    space.length = space.length.saturating_sub(file.length);
                    break;
                }
            }
        }
        spaces.retain(|space| space.length > 0);
        let length = files.iter().map(|f| f.index + f.length).max().unwrap_or(0);
        DiskMap {
            length,
            files,
            spaces,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day9::file_system::tests as fs;

    #[test]
    fn can_compact_fitting_into_spaces() {
        let map = DiskMap::parse("14213");
        let fs = map.compact_fitting_into_spaces().build_file_system();
        assert_eq!(fs::print(&fs), "0222.11")
    }

    #[test]
    fn can_compact_fitting_into_spaces_for_example() {
        let string = "2333133121414131402";
        let map = DiskMap::parse(string);
        let fs = map.compact_fitting_into_spaces().build_file_system();
        assert_eq!(fs::print(&fs), "00992111777.44.333....5555.6666.....8888")
    }
}
