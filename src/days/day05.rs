use crate::utils::io::read_rows;

fn parse_crate_stacks(_rows: &Vec<String>) -> Vec<String> {
    let mut crate_stacks: Vec<String> = Vec::new();

    // Instantiate N crate stacks
    let n_stacks: i32 = (_rows[0].len() as i32 + 1) / 4;
    for _ in 0..n_stacks {
        crate_stacks.push(String::new());
    }

    // Populate stacks
    for row in _rows {
        if row.is_empty() {
            // This means the stacks are finished
            return crate_stacks;
        }

        for stack_id in 0..n_stacks {
            let crate_char_idx: i32 = stack_id * 4 + 1;
            let crate_opener_char: char = row.chars().nth((crate_char_idx - 1) as usize).unwrap();
            let crate_name: char = row.chars().nth(crate_char_idx as usize).unwrap();

            match crate_name {
                ' ' => {}
                _ => {
                    if crate_opener_char == '[' {
                        // Avoid stack index line
                        crate_stacks[stack_id as usize].push(crate_name);
                    }
                }
            }
        }
    }

    crate_stacks
}

#[test]
fn test_parse_crate_stacks() {
    let _rows: Vec<String> = read_rows(&"data/day05.test.txt".to_string());
    let crate_stacks: Vec<String> = parse_crate_stacks(&_rows);
    assert_eq!(crate_stacks[0], "NZ".to_string());
    assert_eq!(crate_stacks[1], "DCM".to_string());
    assert_eq!(crate_stacks[2], "P".to_string());
}

pub fn pt1(file_path: String) -> String {
    let _rows: Vec<String> = read_rows(&file_path);

    let mut crate_stacks: Vec<String> = parse_crate_stacks(&_rows);

    let mut passed_stack_def: bool = false;
    for row in _rows {
        if !passed_stack_def {
            // skip stack definition
            passed_stack_def = row.is_empty();
        } else {
            let instructions: Vec<String> = row.split(' ').into_iter().map(String::from).collect();
            let quantity: usize = instructions[1].parse::<usize>().unwrap();
            let from_idx: usize = instructions[3].parse::<usize>().unwrap() - 1;
            let to_idx: usize = instructions[5].parse::<usize>().unwrap() - 1;

            for _ in 0..quantity {
                // Select crates to move
                let to_move: String = crate_stacks[from_idx][..1].to_string();

                // Remove crates from source stack
                crate_stacks[from_idx] = crate_stacks[from_idx][1..].to_string();

                // Add crates to target stack
                crate_stacks[to_idx] = to_move + &crate_stacks[to_idx];
            }
        }
    }

    // Select top crates
    let mut top_crates: String = String::new();
    for stack in crate_stacks {
        top_crates.push(stack.chars().next().unwrap());
    }

    top_crates
}

#[test]
fn test_pt1() {
    let answer: String = pt1("data/day05.test.txt".to_string());
    assert_eq!(answer, "CMZ");
}

pub fn pt2(file_path: String) -> String {
    let _rows: Vec<String> = read_rows(&file_path);

    let mut crate_stacks: Vec<String> = parse_crate_stacks(&_rows);

    let mut passed_stack_def: bool = false;
    for row in _rows {
        if !passed_stack_def {
            // skip stack definition
            passed_stack_def = row.is_empty();
        } else {
            let instructions: Vec<String> = row.split(' ').into_iter().map(String::from).collect();
            let quantity: usize = instructions[1].parse::<usize>().unwrap();
            let from_idx: usize = instructions[3].parse::<usize>().unwrap() - 1;
            let to_idx: usize = instructions[5].parse::<usize>().unwrap() - 1;

            // Select crates to move
            let to_move: String = crate_stacks[from_idx][..quantity].to_string();

            // Remove crates from source stack
            crate_stacks[from_idx] = crate_stacks[from_idx][quantity..].to_string();

            // Add crates to target stack
            crate_stacks[to_idx] = to_move + &crate_stacks[to_idx];
        }
    }

    // Select top crates
    let mut top_crates: String = String::new();
    for stack in crate_stacks {
        top_crates.push(stack.chars().next().unwrap());
    }

    top_crates
}

#[test]
fn test_pt2() {
    let answer: String = pt2("data/day05.test.txt".to_string());
    assert_eq!(answer, "MCD");
}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => {
            let top_crates: String = pt1("data/day05.txt".to_string());
            println!("The top crates are: {}", top_crates);
            0
        }
        2 => {
            let top_crates: String = pt2("data/day05.txt".to_string());
            println!("The top crates are: {}", top_crates);
            0
        }
        _ => panic!("Part {} not found.", part),
    };
}
