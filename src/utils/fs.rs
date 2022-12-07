pub struct Folder {
    pub files: Vec<File>,
    pub folders: Vec<Folder>,
}

impl Folder {
    fn new() -> Folder {
        Folder {
            files: Vec::new(),
            folders: Vec::new(),
        }
    }

    fn size(&self) -> usize {
        let mut size: usize = 0;
        for file in &self.files {
            size += file.size;
        }
        size
    }
}

pub struct File {
    size: usize,
}

pub fn commands2fs(_rows: &Vec<String>) -> Folder {
    let root: Folder = Folder::new();
    root
}

#[test]
fn test_commands2fs() {
    use crate::utils::io::read_rows;
    let _rows: Vec<String> = read_rows("data/day07.test.txt".to_string());
}
