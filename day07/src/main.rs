use std::time::Instant;

// use the advent package
use advent;
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
    let start = Instant::now();
    let result1 = part1(&contents);
    let duration = start.elapsed();
    println!("Part 1:\n{}\n\tTook {:?}", result1, duration);

    let start = Instant::now();
    let result2 = part2(&contents);
    let duration = start.elapsed();
    println!("Part 2:\n{}\n\tTook {:?}", result2, duration);
}

// turn off warning for unused variables
#[allow(unused_variables)]
pub fn part1(contents: &String) -> String {
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let mut sum = 0;
    for line in lines {
        sum += test_line(line);
    }
    sum.to_string()
}

fn test_line(line: &str) -> i64 {
    let parts = line.split(":").map(|x| x.trim()).collect::<Vec<&str>>();
    let answer = parts[0].parse::<i64>().unwrap();
    let operands = parts[1].split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
    if check_answer(answer, operands) {
        return answer;
    }
    0
}

fn check_answer(answer: i64, operands: Vec<i64>) -> bool {
    let length = operands.len();
    if length == 1 {
        return operands[0] == answer;
    }
    if check_answer(answer - operands[length-1], operands[0..length-1].to_vec()) {
        return true;
    }
    if (answer % operands[length-1] == 0) && (check_answer(answer / operands[length-1], operands[0..length-1].to_vec())) {
        return true;
    }
    false
}

#[allow(unused_variables)]
pub fn part2(contents: &String) -> String {
    2.to_string()
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
        let contents = advent::read_file_to_string("files/test");
        // call part2 with the contents of the file
        let result = part2(&contents);
        // get the contents of the file "files/test_answer_2"
        let answer = advent::read_file_to_string("files/test_answer_2");
        // compare the result with the answer
        assert_eq!(result, answer);
    }
}
