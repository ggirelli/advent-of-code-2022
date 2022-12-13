use std::collections::HashMap;

fn line_is_command(line: &str) -> bool {
    line.chars().next().unwrap() == '$'
}

#[test]
fn test_line_is_command() {
    assert!(line_is_command(&"$ ls".to_string()));
    assert!(!line_is_command(&"dir a".to_string()));
}

pub fn parse_command(line: &String) -> [String; 2] {
    assert!(line_is_command(line));

    let command_parts: Vec<String> = line[2..].split(' ').into_iter().map(String::from).collect();
    match command_parts.len() {
        1 => [command_parts[0].to_string(), "".to_string()],
        2 => [command_parts[0].to_string(), command_parts[1].to_string()],
        _ => panic!("ERROR: too many command parts '{}'", line),
    }
}

#[test]
fn test_parse_command() {
    assert_eq!(parse_command(&String::from("$ cd a")), ["cd", "a"]);
    assert_eq!(parse_command(&String::from("$ ls a")), ["ls", "a"]);
}

#[test]
#[should_panic]
fn test_parse_command_not_command() {
    parse_command(&String::from("cd a"));
}

#[test]
#[should_panic]
fn test_parse_command_many_parts() {
    parse_command(&String::from("$ cd a a"));
}

pub struct Folder {
    folders: HashMap<String, Folder>,
    files: HashMap<String, usize>,
}

impl Folder {
    pub fn new() -> Folder {
        Folder {
            folders: HashMap::new(),
            files: HashMap::new(),
        }
    }

    pub fn size(&self) -> usize {
        let mut size: usize = 0;
        for file_size in self.files.values() {
            size += file_size;
        }
        for folder in self.folders.values() {
            size += folder.size();
        }
        size
    }

    pub fn iter_folders(&self) -> Vec<&Folder> {
        let mut all_folders: Vec<&Folder> = Vec::new();
        for folder in self.folders.values() {
            all_folders.extend(folder.iter_folders());
        }
        all_folders.extend(self.folders.values());
        all_folders
    }
}

pub fn total_size_small_folders(root: Folder, size_thr: usize) -> usize {
    let mut total_size: usize = 0;
    for folder in root.iter_folders() {
        let current_folder_size: usize = folder.size();
        if current_folder_size < size_thr {
            total_size += current_folder_size;
        }
    }
    total_size
}

pub fn find_smallest_large_folder(root: Folder, size_thr: usize) -> usize {
    let mut min_size: usize = root.size();
    for folder in root.iter_folders() {
        let current_folder_size: usize = folder.size();
        if current_folder_size >= size_thr && min_size > current_folder_size {
            min_size = current_folder_size;
        }
    }
    min_size
}

pub fn commands2fs(_lines: Vec<String>, mut root: Folder) -> (Folder, usize) {
    let mut line_idx: usize = 0;
    while line_idx < _lines.len() {
        let line: &String = &_lines[line_idx];

        if line_is_command(line) {
            let parsed: [String; 2] = parse_command(line);

            match parsed[0].as_str() {
                "cd" => match parsed[1].as_str() {
                    ".." => return (root, line_idx),
                    _ => {
                        let mut selected_dir: Folder = root
                            .folders
                            .remove(&parsed[1])
                            .expect("ERROR: trying to access a missing folder.");

                        let parsed_lines_counter: usize;
                        (selected_dir, parsed_lines_counter) =
                            commands2fs(_lines[(line_idx + 1)..].to_vec(), selected_dir);
                        line_idx += parsed_lines_counter + 1;
                        root.folders.insert(parsed[1].to_string(), selected_dir);
                    }
                },
                "ls" => {}
                _ => panic!("ERROR: unrecognized command line '{}'", line),
            }
        } else {
            let parsed: Vec<String> = line.split(' ').into_iter().map(String::from).collect();

            match parsed[0].as_str() {
                "dir" => {
                    root.folders.insert(parsed[1].to_string(), Folder::new());
                }
                _ => {
                    root.files
                        .insert(parsed[1].to_string(), parsed[0].parse::<usize>().unwrap());
                }
            }
        }

        line_idx += 1;
    }

    (root, line_idx)
}

#[test]
fn test_commands2fs() {
    use crate::utils::io::read_rows;

    let _rows: Vec<String> = read_rows(&"data/day07.test.txt".to_string());
    assert_eq!(_rows[0], "$ cd /");

    let root: Folder;
    (root, _) = commands2fs(_rows[1..].to_vec(), Folder::new());

    assert_eq!(root.size(), 48381165);
    assert_eq!(
        root.folders
            .get("d")
            .expect("ERROR: folder 'd' not found.")
            .size(),
        24933642
    );
    assert_eq!(
        root.folders
            .get("a")
            .expect("ERROR: folder 'a' not found.")
            .size(),
        94853
    );
}
