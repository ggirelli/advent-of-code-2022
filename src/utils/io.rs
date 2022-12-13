use std::fs;

pub fn read_rows(file_path: &String) -> Vec<String> {
    return fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Should have been able to read the file: {}", file_path))
        .split('\n')
        .into_iter()
        .map(String::from)
        .collect();
}
