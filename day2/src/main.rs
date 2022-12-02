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

    calculate_part_one_score(&contents);
    calculate_part_two_score(&contents);
}

fn calculate_part_one_score(contents: &String) {
    let mut total_score = 0;
    // HashMap of (opponent's answer, our answer) -> points awarded
    let mut score_map = std::collections::HashMap::new();

    // A: Rock, B: Paper, C: Scissors
    // X: Rock, Y: Paper, Z: Scissor
    // 1: Rock, 2: Paper, 3: Scissor (points awarded for picking)
    // 0: Loss, 3: Draw, 6: Win
    score_map.insert(("A".to_string(), "X".to_string()), 3);
    score_map.insert(("A".to_string(), "Y".to_string()), 6);
    score_map.insert(("A".to_string(), "Z".to_string()), 0);

    score_map.insert(("B".to_string(), "X".to_string()), 0);
    score_map.insert(("B".to_string(), "Y".to_string()), 3);
    score_map.insert(("B".to_string(), "Z".to_string()), 6);

    score_map.insert(("C".to_string(), "X".to_string()), 6);
    score_map.insert(("C".to_string(), "Y".to_string()), 0);
    score_map.insert(("C".to_string(), "Z".to_string()), 3);

    for line in contents.lines() {
        let elf_answer = line.chars().nth(0).unwrap().to_string();
        let our_answer = line.chars().nth(2).unwrap().to_string();

        // Prob could put these in a better data structure like a vector/map again
        if our_answer == "X" {
            total_score += 1
        }
        else if our_answer == "Y" {
            total_score += 2
        }
        else {
            total_score += 3
        }

        let score = score_map.get(&(elf_answer, our_answer)).unwrap();

        total_score += score;
    }

    println!("Part one total score: {}", total_score);

}

fn calculate_part_two_score(contents: &String) {
    let mut total_score = 0;

    let mut instructed_answer_map = std::collections::HashMap::new();
    instructed_answer_map.insert(("A".to_string(), "X".to_string()), "Z".to_string());
    instructed_answer_map.insert(("A".to_string(), "Y".to_string()), "X".to_string());
    instructed_answer_map.insert(("A".to_string(), "Z".to_string()), "Y".to_string());

    instructed_answer_map.insert(("B".to_string(), "X".to_string()), "X".to_string());
    instructed_answer_map.insert(("B".to_string(), "Y".to_string()), "Y".to_string());
    instructed_answer_map.insert(("B".to_string(), "Z".to_string()), "Z".to_string());

    instructed_answer_map.insert(("C".to_string(), "X".to_string()), "Y".to_string());
    instructed_answer_map.insert(("C".to_string(), "Y".to_string()), "Z".to_string());
    instructed_answer_map.insert(("C".to_string(), "Z".to_string()), "X".to_string());

    let mut score_map = std::collections::HashMap::new();
    score_map.insert(("A".to_string(), "X".to_string()), 3);
    score_map.insert(("A".to_string(), "Y".to_string()), 6);
    score_map.insert(("A".to_string(), "Z".to_string()), 0);

    score_map.insert(("B".to_string(), "X".to_string()), 0);
    score_map.insert(("B".to_string(), "Y".to_string()), 3);
    score_map.insert(("B".to_string(), "Z".to_string()), 6);

    score_map.insert(("C".to_string(), "X".to_string()), 6);
    score_map.insert(("C".to_string(), "Y".to_string()), 0);
    score_map.insert(("C".to_string(), "Z".to_string()), 3);

    for line in contents.lines() {
        let elf_answer = line.chars().nth(0).unwrap().to_string();
        let elf_answer_score = elf_answer.clone();
        let our_outcome = line.chars().nth(2).unwrap().to_string();  // Since x,y,z is lose,draw,win
        let our_answer = instructed_answer_map.get(&(elf_answer, our_outcome)).unwrap().to_string();

        // Prob could put these in a better data structure like a vector/map again
        if our_answer == "X" {
            total_score += 1
        }
        else if our_answer == "Y" {
            total_score += 2
        }
        else {
            total_score += 3
        }

        let score = score_map.get(&(elf_answer_score, our_answer)).unwrap();

        total_score += score;
    }

    println!("Part two total score: {}", total_score);
}
