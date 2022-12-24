use std::fs;

pub fn read_rows(file_path: &String) -> Vec<String> {
    return fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Should have been able to read the file: {}", file_path))
        .split('\n')
        .into_iter()
        .map(String::from)
        .collect();
}

pub fn group_row_pairs(_rows: &Vec<String>, sep: String) -> Vec<(String, String)> {
    let mut row_groups: Vec<(String, String)> = Vec::new();
    let mut current_group: Vec<String> = Vec::new();
    for row in _rows {
        if row == &sep {
            assert_eq!(current_group.len(), 2);
            row_groups.push((current_group[0].to_string(), current_group[1].to_string()));
            current_group = Vec::new();
        } else {
            current_group.push(row.to_string());
        }
    }
    if current_group.len() > 0 {
        assert_eq!(current_group.len(), 2);
        row_groups.push((current_group[0].to_string(), current_group[1].to_string()));
    }
    row_groups
}
