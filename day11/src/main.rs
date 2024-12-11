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
    let stones: Vec<&str> = contents.split_whitespace().collect();
    let mut new_stones: Vec<String> = stones.iter().map(|&s| s.to_string()).collect();
    // print_stones(&new_stones);
    for _ in 0..25 {
        new_stones = blink(new_stones);
        // print_stones(&new_stones);
    }
    new_stones.len().to_string()
}

#[allow(dead_code)]
fn print_stones(stones: &Vec<String>) {
    for stone in stones.iter() {
        print!("({}) ", stone);
    }
    println!();
}

fn blink(stones: Vec<String>) -> Vec<String> {
    let mut new_stones: Vec<String> = Vec::new();
    for stone in stones.iter() {
        if *stone == "0" {
            new_stones.push("1".to_string());
        } else if stone.len() % 2 == 0 {
            let split_stones = stone.split_at(stone.len()/2);
            let v1 = split_stones.0.parse::<u64>().unwrap();
            let v2 = split_stones.1.parse::<u64>().unwrap();
            new_stones.push(v1.to_string());
            new_stones.push(v2.to_string());
        } else {
            let stone_value = stone.parse::<u64>().unwrap();
            let new_stone = (stone_value * (2024 as u64)).to_string();
            new_stones.push(new_stone);
        }
    }

    new_stones
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
