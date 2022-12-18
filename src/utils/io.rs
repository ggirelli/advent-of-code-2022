use std::fs;

pub fn read_rows(file_path: &String) -> Vec<String> {
    return fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Should have been able to read the file: {}", file_path))
        .split('\n')
        .into_iter()
        .map(String::from)
        .collect();
}

pub fn group_rows(_rows: &Vec<String>, sep: String) -> Vec<Vec<String>> {
    let mut row_groups: Vec<Vec<String>> = Vec::new();
    let mut current_group: Vec<String> = Vec::new();
    for row in _rows {
        if row == &sep {
            row_groups.push(current_group);
            current_group = Vec::new();
        } else {
            current_group.push(row.to_string());
        }
    }
    if current_group.len() > 0 {
        row_groups.push(current_group);
    }
    row_groups
}
