use std::{collections::HashMap, time::Instant};

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
    let stones: Vec<&str> = contents.split_whitespace().collect();

    let mut stone_count: StoneCount = HashMap::new();
    for stone in stones.iter() {
        let count = stone_count.entry(stone.to_string()).or_insert(0);
        *count += 1;
    }

    // print_stones(&new_stones);
    for _ in 0..75 {
        stone_count = better_blink(&mut stone_count);
        // print_stones(&new_stones);
    }

    let mut total: u64 = 0;
    for (stone, count) in stone_count.iter() {
        total += *count as u64;
    }
    total.to_string()

}

type StoneCount = HashMap<String, u64>;

fn better_blink(stone_count: &StoneCount) -> StoneCount{
    let mut new_stone_count: StoneCount = HashMap::new();
    for (stone, count) in stone_count.iter() {
        if *stone == "0" {
            let new_count = new_stone_count.entry("1".to_string()).or_insert(0);
            *new_count += *count;
        } else if stone.len() % 2 == 0 {
            let split_stones = stone.split_at(stone.len()/2);
            let v1 = split_stones.0.parse::<u64>().unwrap();
            let v2 = split_stones.1.parse::<u64>().unwrap();
            let new_count1 = new_stone_count.entry(v1.to_string()).or_insert(0);
            *new_count1 += *count;
            let new_count2 = new_stone_count.entry(v2.to_string()).or_insert(0);
            *new_count2 += *count;
        } else {
            let stone_value = stone.parse::<u64>().unwrap();
            let new_stone = (stone_value * (2024 as u64)).to_string();
            let new_count = new_stone_count.entry(new_stone).or_insert(0);
            *new_count += *count;
        }
    }
    new_stone_count
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
