use std::fs;

fn main() {
    let file_path = "../data/day1.txt";
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
            println!("Elf #{} has {} calories.", elf_counter+1, elf_calories[elf_counter]);
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
    println!("The elf with most calories has: {}", elf_calories[0]);
    
    let top_three_elves = &elf_calories[0..3];
    let top_three_elves_sum: i32 = top_three_elves.iter().sum();
    println!("The three elves carrying most calories have a total of {} calories.", top_three_elves_sum);
}
