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
    // part_two(&contents);
}

fn part_one(contents: &String) {
}
