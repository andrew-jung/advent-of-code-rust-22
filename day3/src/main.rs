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
    part_one(&contents)
}

fn part_one(contents: &String) {
    // Each compartment is separated into two; equal halves.
    // Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.

    // Find the item type that appears in both compartments of each rucksack.
    // What is the sum of the priorities of those item types?

    // For each line in contents find the common character(s):
    // 1. Split the line into two halves
    // 2. Find the common characters between the two halves
    // 3. Add the priority of the common character to the total

    let mut total = 0;
    for line in contents.lines() {

        let line_length = line.len();
        let first_half = &line[0..line_length/2];
        let second_half = &line[line_length/2..line_length];

        // Count each common charaacter only once
        let common_chars = first_half.chars().filter(|c| second_half.contains(*c)).collect::<Vec<char>>();
        let mut unique_chars = common_chars.clone();
        unique_chars.dedup();
        for c in unique_chars {
            let priority = match c {
                'a'..='z' => (c as u32) - ('a' as u32) + 1,
                'A'..='Z' => (c as u32) - ('A' as u32) + 27,
                _ => panic!("Unexpected character")
            };
            total += priority;
        }
    }

    println!("Part one: {}", total);
}

fn part_two(contents: &String) {

}
