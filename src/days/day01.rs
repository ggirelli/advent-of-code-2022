use crate::utils::io::read_rows;

pub fn pt1() {
    let _rows: Vec<String> = read_rows(&"data/day01.txt".to_string());

    // Setup informative counters
    let mut row_counter: i32 = 0;
    let mut calories_item_counter: i32 = 0;
    let mut elf_counter: i32 = 1;

    // Retain only highest elf calorie count
    let mut max_elf_calories: i32 = 0;
    let mut current_elf_calories: i32 = 0;
    for row in _rows {
        if row == "" {
            // Empty line indicates the current elf is done
            // println!("Elf #{elf_counter} has {current_elf_calories} calories. (max has {max_elf_calories})");
            if current_elf_calories >= max_elf_calories {
                max_elf_calories = current_elf_calories;
            }
            current_elf_calories = 0;
            elf_counter += 1;
        } else {
            // Otherwise keep summing calories
            calories_item_counter += 1;
            current_elf_calories += row.parse::<i32>().unwrap();
        }

        row_counter += 1;
    }

    // Since the file does not end with an empty line, check the last elf
    if current_elf_calories >= max_elf_calories {
        max_elf_calories = current_elf_calories;
    }

    println!(
        "With {row_counter} rows, covering {elf_counter} elfs with {calories_item_counter} items."
    );
    println!("The elf with most calories has: {max_elf_calories}")
}

pub fn pt2() {
    let _rows: Vec<String> = read_rows(&"data/day01.txt".to_string());

    let mut elf_counter: usize = 0;
    let mut elf_calories: Vec<i32> = Vec::new();
    let mut elf_items_counter: Vec<i32> = Vec::new();
    elf_calories.push(0);
    elf_items_counter.push(0);

    for row in _rows {
        if row == "" {
            // Empty line indicates the current elf is done
            // println!("Elf #{} has {} calories.", elf_counter+1, elf_calories[elf_counter]);
            elf_counter += 1;
            elf_calories.push(0);
            elf_items_counter.push(0);
        } else {
            // Otherwise keep summing calories
            elf_items_counter[elf_counter] += 1;
            elf_calories[elf_counter] += row.parse::<i32>().unwrap();
        }
    }

    elf_calories.sort();
    elf_calories.reverse();
    // println!("The elf with most calories has: {}", elf_calories[0]);

    let top_three_elves: &[i32] = &elf_calories[0..3];
    let top_three_elves_sum: i32 = top_three_elves.iter().sum();
    println!(
        "The three elves carrying most calories have a total of {} calories.",
        top_three_elves_sum
    );
}

pub fn run(part: i32) {
    match part {
        1 => pt1(),
        2 => pt2(),
        _ => panic!("Part {} not found.", part),
    }
}
