use std::collections::HashMap;

pub fn build_letter_value_map() -> HashMap<char, i32> {
    let alphabet: String = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQTRSUVWXYZ".to_string();
    let mut letter_values: HashMap<char, i32> = HashMap::new();
    for letter in alphabet.chars() {
        let mut value: i32 = letter as i32;
        if value >= 97 {
            value -= 96;
        } else {
            value -= 38;
        }
        letter_values.insert(letter, value);
    }
    letter_values
}

#[test]
fn test_build_letter_value_map() {
    let letter_values = build_letter_value_map();
    match letter_values.get(&'p') {
        Some(&value) => assert_eq!(value, 16),
        _ => assert_eq!(0, 16),
    }
    match letter_values.get(&'L') {
        Some(&value) => assert_eq!(value, 38),
        _ => assert_eq!(0, 38),
    }
    match letter_values.get(&'P') {
        Some(&value) => assert_eq!(value, 42),
        _ => assert_eq!(0, 42),
    }
    match letter_values.get(&'v') {
        Some(&value) => assert_eq!(value, 22),
        _ => assert_eq!(0, 22),
    }
    match letter_values.get(&'t') {
        Some(&value) => assert_eq!(value, 20),
        _ => assert_eq!(0, 20),
    }
    match letter_values.get(&'s') {
        Some(&value) => assert_eq!(value, 19),
        _ => assert_eq!(0, 19),
    }
}
