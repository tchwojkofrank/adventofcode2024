use std::{collections::HashMap, time::Instant};
use regex;

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
    let sections: Vec<&str> = contents.split("\n\n").collect();
    let towels = sections[0];
    let designs = sections[1].split("\n").collect::<Vec<&str>>();
    let mut count = 0;
    for design in designs {
        if is_design_possible(towels, design) {
            count += 1;
        }
    }
    count.to_string()
}

fn is_design_possible(towels: &str, design: &str) -> bool {
    let mut towel_regex = towels.to_string().replace(", ","|");
    towel_regex = "^(".to_string() + &towel_regex + ")+$";
    let re = regex::Regex::new(&towel_regex).unwrap();
    // let a_match = re.find(design);
    // if a_match.is_some() {
    //     println!("{} is possible for {}", a_match.unwrap().as_str(), design);
    // }
    re.is_match(design)
}

#[allow(unused_variables)]
pub fn part2(contents: &String) -> String {
    let sections: Vec<&str> = contents.split("\n\n").collect();
    let towels = sections[0];
    let designs = sections[1].split("\n").collect::<Vec<&str>>();
    let mut count = 0 as u64;
    let towel_array: Vec<&str> = towels.split(", ").collect();
    let towel_slice = &towel_array[..];
    let mut solution_map: HashMap<String,u64> = std::collections::HashMap::new();
    for design in designs {
        if is_design_possible(towels, design) {
            count += possible_solutions(towel_slice, design, &mut solution_map);
        }
        // println!("Design: {} Count: {}", design, count);
    }
    count.to_string()
}

fn possible_solutions(towels: &[&str], design: &str, solution_map: &mut HashMap<String, u64>) -> u64 {
    let mut solutions = 0;
    if solution_map.contains_key(design) {
        return solution_map[design];
    }
    for towel in towels {
        if design.starts_with(towel) {
            let remaining = &design[towel.len()..];
            if remaining.len() == 0 {
                solutions += 1;
            } else {
                solutions += possible_solutions(towels, remaining, solution_map);
            }
        }
    }
    solution_map.insert(design.to_string(), solutions);
    solutions
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
