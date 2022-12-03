use std::collections::HashMap;

pub fn halve_string(word: String) -> [String; 2] {
    assert_eq!(word.len() % 2, 0);
    [
        word[0..(word.len() / 2)].to_string(),
        word[(word.len() / 2)..word.len()].to_string(),
    ]
}

#[test]
fn test_halve_string_even_word() {
    assert_eq!(
        halve_string("asdfgf".to_string()),
        ["asd".to_string(), "fgf".to_string()]
    );
}

#[test]
#[should_panic]
fn test_halve_string_odd_word() {
    let _halves: [String; 2] = halve_string("asdfgfa".to_string());
}

pub fn _count_characters(word: String) -> HashMap<char, i32> {
    let mut char_counts: HashMap<char, i32> = HashMap::new();
    for letter in word.chars() {
        let letter_count: &mut i32 = char_counts.entry(letter).or_insert(0);
        *letter_count += 1;
    }
    char_counts
}

#[test]
fn test_count_characters() {
    assert_eq!(
        _count_characters("asdfgf".to_string()),
        HashMap::from([('a', 1), ('s', 1), ('d', 1), ('f', 2), ('g', 1)])
    );
}

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
        // println!("{} => {}", letter, value);
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

pub fn get_common_chars(first_word: String, second_word: String) -> Vec<char> {
    let mut common_chars: Vec<char> = Vec::new();
    for letter in first_word.chars() {
        if second_word.contains(letter) & (!common_chars.contains(&letter)) {
            common_chars.push(letter);
        }
    }
    common_chars
}

#[test]
fn test_get_common_chars() {
    assert_eq!(
        get_common_chars("vJrwpWtwJgWr".to_string(), "hcsFMMfFFhFp".to_string())[0],
        'p'
    );
    assert_eq!(
        get_common_chars(
            "jqHRNqRjqzjGDLGL".to_string(),
            "rsFMfFZSrLrFZsSL".to_string()
        )[0],
        'L'
    );
    assert_eq!(
        get_common_chars("PmmdzqPrV".to_string(), "vPwwTWBwg".to_string())[0],
        'P'
    );
}
