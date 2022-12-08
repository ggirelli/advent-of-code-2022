use crate::utils::io::read_rows;
use crate::utils::vec::do_overlap_i32;
use crate::utils::vec::is_subset_i32;

fn assignment2sections(assignment: String) -> Vec<i32> {
    let mut sections: Vec<i32> = Vec::new();
    let boundaries: Vec<i32> = assignment
        .split("-")
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    for section_id in boundaries[0]..(boundaries[1] + 1) {
        sections.push(section_id);
    }
    sections
}

pub fn pt1(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(&file_path);

    let mut subset_pair_counter: i32 = 0;
    for row in _rows {
        let elf_assignments: Vec<String> = row.split(",").into_iter().map(String::from).collect();
        let first_elf_sections: Vec<i32> = assignment2sections(elf_assignments[0].to_string());
        let second_elf_sections: Vec<i32> = assignment2sections(elf_assignments[1].to_string());

        // Check if first elf is subset of second
        if is_subset_i32(&first_elf_sections, &second_elf_sections) {
            subset_pair_counter += 1;
        }

        // If elves have different number of sections assigned
        // Also check if second elf is subset of first
        let same_size: bool = first_elf_sections.len() == second_elf_sections.len();
        if is_subset_i32(&second_elf_sections, &first_elf_sections) & !same_size {
            subset_pair_counter += 1;
        }
    }

    subset_pair_counter
}

#[test]
fn test_pt1() {
    let answer: i32 = pt1("data/day04.test.txt".to_string());
    assert_eq!(answer, 2);
}
pub fn pt2(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(&file_path);

    let mut overlap_pair_counter: i32 = 0;
    for row in _rows {
        let elf_assignments: Vec<String> = row.split(",").into_iter().map(String::from).collect();
        let first_elf_sections: Vec<i32> = assignment2sections(elf_assignments[0].to_string());
        let second_elf_sections: Vec<i32> = assignment2sections(elf_assignments[1].to_string());

        // Check if first elf is subset of second
        if do_overlap_i32(&first_elf_sections, &second_elf_sections) {
            overlap_pair_counter += 1;
        }
    }

    overlap_pair_counter
}

#[test]
fn test_pt2() {
    let answer: i32 = pt2("data/day04.test.txt".to_string());
    assert_eq!(answer, 4);
}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => {
            let subset_counter: i32 = pt1("data/day04.txt".to_string());
            println!(
                "The number of assignment pairs where one range fully contain the other is {}",
                subset_counter
            );
            subset_counter
        }
        2 => {
            let overlap_counter: i32 = pt2("data/day04.txt".to_string());
            println!(
                "The number of assignment pairs with overlap is {}",
                overlap_counter
            );
            overlap_counter
        }
        _ => {
            println!("Part {} not found.", part);
            0
        }
    };
}
