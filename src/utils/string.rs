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

pub fn get_common_characters(first_word: String, second_word: String) -> Vec<char> {
    let mut common_chars: Vec<char> = Vec::new();
    for letter in first_word.chars() {
        if second_word.contains(letter) & (!common_chars.contains(&letter)) {
            common_chars.push(letter);
        }
    }
    common_chars
}

#[test]
fn test_get_common_characters() {
    assert_eq!(
        get_common_characters("vJrwpWtwJgWr".to_string(), "hcsFMMfFFhFp".to_string())[0],
        'p'
    );
    assert_eq!(
        get_common_characters(
            "jqHRNqRjqzjGDLGL".to_string(),
            "rsFMfFZSrLrFZsSL".to_string()
        )[0],
        'L'
    );
    assert_eq!(
        get_common_characters("PmmdzqPrV".to_string(), "vPwwTWBwg".to_string())[0],
        'P'
    );
}

pub fn count_characters(word: String) -> HashMap<char, i32> {
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
        count_characters("asdfgf".to_string()),
        HashMap::from([('a', 1), ('s', 1), ('d', 1), ('f', 2), ('g', 1)])
    );
}

pub fn unique_string(word: String) -> String {
    let mut unique_characters: Vec<char> = Vec::new();
    for letter in count_characters(word).keys() {
        unique_characters.push(*letter);
    }
    unique_characters.sort();

    String::from_iter(unique_characters)
}

#[test]
fn test_unique_string() {
    assert_eq!(
        unique_string("kahsfdCHSAILCaslhid".to_string()),
        "ACHILSadfhikls".to_string()
    )
}

pub fn find_stretch_of_unique_characters(content: String, length: usize) -> i32 {
    for i in (length - 1)..content.len() {
        let mut has_repeats: bool = false;
        for count in count_characters(content[(i + 1 - length)..(i + 1)].to_string()).values() {
            has_repeats = *count > 1;
            if has_repeats {
                break;
            }
        }
        if !has_repeats {
            return (i + 1) as i32;
        }
    }
    -1
}

#[test]
fn test_find_stretch_of_unique_characters() {
    assert_eq!(
        find_stretch_of_unique_characters("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 4),
        7
    );
    assert_eq!(
        find_stretch_of_unique_characters("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 4),
        5
    );
    assert_eq!(
        find_stretch_of_unique_characters("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 4),
        6
    );
    assert_eq!(
        find_stretch_of_unique_characters("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 4),
        10
    );
    assert_eq!(
        find_stretch_of_unique_characters("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 4),
        11
    );

    assert_eq!(
        find_stretch_of_unique_characters("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 14),
        19
    );
    assert_eq!(
        find_stretch_of_unique_characters("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 14),
        23
    );
    assert_eq!(
        find_stretch_of_unique_characters("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 14),
        23
    );
    assert_eq!(
        find_stretch_of_unique_characters("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 14),
        29
    );
    assert_eq!(
        find_stretch_of_unique_characters("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 14),
        26
    );
}
