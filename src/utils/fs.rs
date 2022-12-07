use std::collections::HashMap;

pub struct Folder {
    pub files: HashMap<String, File>,
    pub folders: HashMap<String, Folder>,
}

impl Folder {
    fn new() -> Folder {
        Folder {
            files: HashMap::new(),
            folders: HashMap::new(),
        }
    }

    fn size(&self) -> usize {
        let mut size: usize = 0;
        for file in self.files.values() {
            size += file.size;
        }
        for folder in self.folders.values() {
            size += folder.size();
        }
        size
    }

    fn get_child_folder(&self, path: Vec<String>) -> &Folder {
        let mut current_folder: &Folder = &self;
        for path_part in &path {
            assert!(current_folder.folders.contains_key(path_part));
            match current_folder.folders.get(path_part) {
                Some(target_folder) => current_folder = target_folder,
                _ => panic!("ERROR: {:?} folder not found.", path),
            }
        }
        current_folder
    }

    fn get_child_file(&self, path: Vec<String>) -> &File {
        match self.get_child_folder(path[..(path.len()-1)].to_vec()).files.get(&path[path.len()-1]) {
            Some(child_file) => {return child_file;},
            _ => panic!("ERROR: {:?} file not found.", path),
        }
    }
}

pub struct File {
    size: usize,
}

fn line_is_command(line: &String) -> bool {
    line.chars().nth(0).unwrap() == '$'
}

#[test]
fn test_line_is_command() {
    assert_eq!(line_is_command(&"$ ls".to_string()), true);
    assert_eq!(line_is_command(&"dir a".to_string()), false);
}

pub fn commands2fs(_lines: &Vec<String>) -> Folder {
    assert_eq!(line_is_command(&_lines[0]), true);
    assert_eq!(_lines[0][2..4].to_string(), "cd");
    let mut root: Folder = Folder::new();

    let mut folder_path: Vec<String> = Vec::new();
    let mut current_command: String = String::from("");
    let mut command_output: Vec<String> = Vec::new();
    for line in _lines[1..].to_vec() {
        if line_is_command(&line) {
            if current_command.len() > 0 {
                // Parse command output

                // Reset command output
                command_output = Vec::new();
            }

            current_command = line.to_string();
            match &line[2..4] {
                "cd" => {
                    let target: String = line[5..].to_string();
                    if target == "..".to_string() {
                        assert!(folder_path.len() > 0);
                        folder_path = folder_path[..(folder_path.len() - 1)].to_vec();
                    } else {
                        root.folders.insert(target.to_string(), Folder::new());
                        folder_path.push(target);
                    }
                },
                "ls" => {},
                _ => {},
            }
        } else {
            command_output.push(line.to_string());
        }
    }

    root
}

#[test]
fn test_commands2fs() {
    use crate::utils::io::read_rows;
    let _rows: Vec<String> = read_rows("data/day07.test.txt".to_string());
}
