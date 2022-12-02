use std::collections::BinaryHeap;
use std::env;
use std::fs;

struct InputFile {
    file_path: String
}

impl InputFile {
    fn new(args: &[String]) -> InputFile {
        if args.len() < 2{
            panic!("Not enough arguments, please pass in a file path");
        }
        let file_path = args[1].clone();
        InputFile { file_path }
    }
}

fn run(input_file: InputFile) -> String{
    let contents = fs::read_to_string(input_file.file_path)
        .expect("Something went wrong reading the file");

        contents
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = InputFile::new(&args);

    let contents = run(input_file);

    part_one(&contents);
    part_one_heaped(&contents);
    part_two(&contents);
}

fn part_one(contents: &String) {
    println!("Part one");

    let mut max_elf_id = 0;
    let mut max_elf_calories = 0;

    let elf_calories: Vec<_> = contents.split_terminator("\n\n").collect();

    for (index, elf) in elf_calories.iter().enumerate() {
        let total_calories_per_elf: u32 = elf.lines().map(|calorie| calorie.parse::<u32>().unwrap()).sum();
        if total_calories_per_elf > max_elf_calories {
            max_elf_calories = total_calories_per_elf;
            max_elf_id = index + 1;
        }
    }

    println!("Elf {} has the most calories with {}", max_elf_id, max_elf_calories);
}

fn part_one_heaped(contents: &String) {
    println!("Part one w/ heap");
    let mut heap = BinaryHeap::new();

    let elf_calories: Vec<_> = contents.split_terminator("\n\n").collect();
    for elf_or_line_break in elf_calories {
        let total_calories_per_elf: u32 = elf_or_line_break.lines().map(|calorie| calorie.parse::<u32>().unwrap()).sum();
        heap.push(total_calories_per_elf);
    }

    assert_eq!(heap.pop(), Some(67016));
}

fn part_two(contents: &String) {
    println!("Part two");
    // Part one but return top three, use a hashmap or something similar

    let elf_calories: Vec<_> = contents.split_terminator("\n\n").collect();
    let mut elf_map = Vec::new();

    for elf in elf_calories.iter() {
        let total_calories_per_elf: u32 = elf.lines().map(|calorie| calorie.parse::<u32>().unwrap()).sum();
        elf_map.push(total_calories_per_elf);
    }

    elf_map.sort();
    elf_map.reverse();

    let top_three_total = elf_map[0] + elf_map[1] + elf_map[2];
    println!("Top three elves are carrying {} calories ", top_three_total);
}
