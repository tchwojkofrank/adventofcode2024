// use the advent package
use advent;
use regex::Regex;

fn main() {
    let args = advent::get_commandline_arguments();
    // the first argument is the input file name
    // if the user does not provide a first argument, the length of args will be zero
    if args.len() == 0 {
        eprintln!("Usage: day00 <input file>");
        std::process::exit(1);
    }
    // the contents of the file are ascii text. Read the contents of the file into a string
    let filename = &args[0];
    let contents = advent::read_file_to_string(filename);
    // call part1 with the contents of the file
    let result1 = part1(&contents);
    println!("Part 1: {}", result1);
    let result2 = part2(&contents);
    println!("Part 2: {}", result2);
}

// turn off warning for unused variables
#[allow(unused_variables)]
pub fn part1(contents: &String) -> String {
    let instruction_args = get_instruction_args(contents);
    let result = get_instruction_value(&instruction_args);
    result.to_string()
}

fn get_instruction_value(instruction_args: &Vec<(i32, i32)>) -> i32 {
    let mut count = 0;
    for (a, b) in instruction_args {
        count += a * b;
    }
    count
}

fn get_instruction_args(contents: &String) -> Vec<(i32, i32)> {
    // contents is a single string. We are looking for all instances of the following regex:
    // mul(\\d+,\\d+)
    // where \\d+ is one or more digits
    // the regex is looking for the string "mul" followed by an open parenthesis, one or more digits, a comma, one or more digits, and a close parenthesis
    let valid_instruction = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    // create a vector of pairs of integers
    let mut instruction_args: Vec<(i32, i32)> = Vec::new();
    for [arg1, arg2] in valid_instruction.captures_iter(contents).map(|cap| ([cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str()])) {
        instruction_args.push((arg1.parse().unwrap(), arg2.parse().unwrap()));
    }
    instruction_args
}

#[allow(unused_variables)]
pub fn part2(contents: &String) -> String {
    let instruction_args = get_full_instruction_args(contents);
    let result = get_full_instruction_value(&instruction_args);
    result.to_string()
}

enum Instruction {
    IDo,
    IDont,
    IMul(i32, i32),
}

fn get_full_instruction_args(contents: &String) -> Vec<Instruction> {
    let valid_instruction = Regex::new(r"(?<mul>mul)\((\d+),(\d+)\)|(?<do>do)\(\)|(?<dont>don't)\(\)").unwrap();
    let mut instructions: Vec<Instruction> = Vec::new();
    for instruction_capture in valid_instruction.captures_iter(contents) {
        let instruction = instruction_capture.name("mul").or(instruction_capture.name("do")).or(instruction_capture.name("dont"));
        let mut instruction_string = "";
        match instruction {
            None => {
                println!("Invalid instruction");
            },
            Some(_) => {
                instruction_string = instruction.unwrap().as_str();
            },
        }
        match instruction_string {
            "mul" => {
                let arg1 = instruction_capture.get(2).unwrap().as_str().parse().unwrap();
                let arg2 = instruction_capture.get(3).unwrap().as_str().parse().unwrap();
                instructions.push(Instruction::IMul(arg1, arg2));
            },
            "do" => {
                instructions.push(Instruction::IDo);
            },
            "don\'t" => {
                instructions.push(Instruction::IDont);
            },
            _ => {
                println!("Invalid instruction: {}", instruction_string);
            },
        }
    }
    instructions
}

fn get_full_instruction_value(instruction_args: &Vec<Instruction>) -> i32 {
    let mut count = 0;
    let mut enable = true;
    for instruction in instruction_args {
        match instruction {
            Instruction::IMul(a, b) => {
                if enable {
                    count += a * b;
                }
            },
            Instruction::IDo => {
                enable = true;
            },
            Instruction::IDont => {
                enable = false;
            },
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // get the contents of the file "files/test"
        let contents = advent::read_file_to_string("files/test");
        // call part1 with the contents of the file
        let result = part1(&contents);
        // get the contents of the file "files/test_answer_1"
        let answer = advent::read_file_to_string("files/test_answer_1");
        // compare the result with the answer
        assert_eq!(result, answer);
    }

    #[test]
    fn test2() {
        // get the contents of the file "files/test"
        let contents = advent::read_file_to_string("files/test2");
        // call part2 with the contents of the file
        let result = part2(&contents);
        // get the contents of the file "files/test_answer_2"
        let answer = advent::read_file_to_string("files/test_answer_2");
        // compare the result with the answer
        assert_eq!(result, answer);
    }
}
