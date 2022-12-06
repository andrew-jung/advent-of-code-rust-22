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
    part_two(&contents);
}

fn part_one(contents: &String) {
    let mut count_common_ranges = 0;

    // For each line look at the number pairs, like 17-99,18-24
    // Some of the pairs have noticed that one of their assignments fully contains the other. For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6. In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, so these seem like the most in need of reconsideration. In this example, there are 2 such pairs.

    // Iterate through each line and split the line into the two ranges
    // In how many assignment pairs does one range fully contain the other?

    let lines = contents.lines();
    for line in lines {
        let mut ranges = line.split(",");
        let range_one = ranges.next().unwrap();
        let range_two = ranges.next().unwrap();

        let mut range_one_values = range_one.split("-");
        let range_one_start = range_one_values.next().unwrap().parse::<i32>().unwrap();
        let range_one_end = range_one_values.next().unwrap().parse::<i32>().unwrap();

        let mut range_two_values = range_two.split("-");
        let range_two_start = range_two_values.next().unwrap().parse::<i32>().unwrap();
        let range_two_end = range_two_values.next().unwrap().parse::<i32>().unwrap();

        if range_one_start <= range_two_start && range_one_end >= range_two_end {
            count_common_ranges += 1;
        } else if range_two_start <= range_one_start && range_two_end >= range_one_end {
            count_common_ranges += 1;
        }
    }

    println!("Part one: {}", count_common_ranges);
}

fn part_two(contents: &String) {
    let mut count_common_ranges = 0;

    let lines = contents.lines();
    for line in lines {
        let mut ranges = line.split(",");
        let range_one = ranges.next().unwrap();
        let range_two = ranges.next().unwrap();

        let mut range_one_values = range_one.split("-");
        let range_one_start = range_one_values.next().unwrap().parse::<i32>().unwrap();
        let range_one_end = range_one_values.next().unwrap().parse::<i32>().unwrap();

        let mut range_two_values = range_two.split("-");
        let range_two_start = range_two_values.next().unwrap().parse::<i32>().unwrap();
        let range_two_end = range_two_values.next().unwrap().parse::<i32>().unwrap();

        // Check if any of the ranges overlap at all, and how many are there:
        if (range_one_start <= range_two_start && range_one_end >= range_two_start) || (range_two_start <= range_one_start && range_two_end >= range_one_start) {
            count_common_ranges += 1;
        }


    }
    println!("Part two: {}", count_common_ranges);

}
