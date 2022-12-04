use std::fs;

fn main() {
    /********\
    | part 1 |
    \********/
    let file = fs::read_to_string("input.txt").expect("Unable to read file.");

    let mut elves = Vec::new();
    let mut calories = 0;

    for line in file.lines() {
        // new elf
        if line == "" {
            elves.push(calories);
            calories = 0;
        } else {
            calories += line.to_string().parse::<i32>().unwrap();
        }
    }

    // last elf
    elves.push(calories);

    let max = elves.iter().max().unwrap();
    println!("max: {}", max);

    /********\
    | part 2 |
    \********/
    elves.sort();

    let len = elves.len();
    let mut elf: i32;
    let mut sum = 0;

    for i in len - 3..len {
        elf = elves[i];
        sum += elf;
        println!("index {}: {}", i, elf);
    }

    println!("top 3 sum: {}", sum);
}
