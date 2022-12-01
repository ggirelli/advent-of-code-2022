use std::fs;

pub fn pt1() {
    // Read file content
    let file_path = "data/day01.txt";
    println!("Input file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // Prepare to iterate by row
    let row_generator = contents.split("\n");

    // Setup informative counters
    let mut row_counter = 0;
    let mut calories_item_counter = 0;
    let mut elf_counter = 1;
    
    // Retain only highest elf calorie count
    let mut max_elf_calories = 0;
    let mut current_elf_calories = 0;
    for row in row_generator {
        if row == "" { // Empty line indicates the current elf is done
            // println!("Elf #{elf_counter} has {current_elf_calories} calories. (max has {max_elf_calories})");
            if current_elf_calories >= max_elf_calories {
                max_elf_calories = current_elf_calories;
            }
            current_elf_calories = 0;
            elf_counter += 1;

        } else { // Otherwise keep summing calories
            calories_item_counter += 1;
            current_elf_calories += row.parse::<i32>().unwrap();
        }

        row_counter += 1;
    }

    // Since the file does not end with an empty line, check the last elf
    if current_elf_calories >= max_elf_calories {
        max_elf_calories = current_elf_calories;
    }

    println!("With {row_counter} rows, covering {elf_counter} elfs with {calories_item_counter} items.");
    println!("The elf with most calories has: {max_elf_calories}")
}

pub fn pt2() {
    let file_path = "data/day01.txt";
    println!("Input file {}", file_path);

    // Read file content
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // Prepare to iterate by row
    let row_generator =  contents.split("\n");

    let mut elf_counter = 0;
    let mut elf_calories: Vec<i32> = Vec::new();
    let mut elf_items_counter: Vec<i32> = Vec::new();
    elf_calories.push(0);
    elf_items_counter.push(0);

    for row in row_generator {
        if row == "" { // Empty line indicates the current elf is done
            // println!("Elf #{} has {} calories.", elf_counter+1, elf_calories[elf_counter]);
            elf_counter += 1;
            elf_calories.push(0);
            elf_items_counter.push(0);

        } else { // Otherwise keep summing calories
            elf_items_counter[elf_counter] += 1;
            elf_calories[elf_counter] += row.parse::<i32>().unwrap();
        }
    }

    
    elf_calories.sort();
    elf_calories.reverse();
    // println!("The elf with most calories has: {}", elf_calories[0]);
    
    let top_three_elves = &elf_calories[0..3];
    let top_three_elves_sum: i32 = top_three_elves.iter().sum();
    println!("The three elves carrying most calories have a total of {} calories.", top_three_elves_sum);
}
