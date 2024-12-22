use std::{collections::{HashMap, HashSet}, time::Instant};

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
    let secrets = lines.iter().map(|line| line.parse::<u128>().unwrap()).collect::<Vec<u128>>();
    let mut secrets2k = Vec::new();
    let mut sum = 0;
    for secret in secrets.iter() {
        let x = *secret;
        let mut y = x;
        for _ in 0..2000 {
            y = next(y);
        }
        secrets2k.push(y);
        sum += y;
        println!("{}: {}", x, y);
    }
    sum.to_string()
}

fn next(secret: u128) -> u128 {
    step3(step2(step1(secret)))
}

fn step1(secret: u128) -> u128 {
    prune(mix(secret, secret << 6))
}

fn step2(secret: u128) -> u128 {
    prune(mix(secret, secret >> 5))
}

fn step3(secret: u128) -> u128 {
    prune(mix(secret, secret*2048))
}

fn mix(secret: u128, x: u128) -> u128 {
    x ^ secret
}

fn prune(x: u128) -> u128 {
    x & 0xffffff
}

#[allow(unused_variables)]
pub fn part2(contents: &String) -> String {
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let secrets = lines.iter().map(|line| line.parse::<u128>().unwrap()).collect::<Vec<u128>>();
    let mut secrets2klist: Vec<Vec<i32>> = Vec::new();
    for secret in secrets.iter() {
        let mut secrets2k = Vec::new();
        let mut y = *secret;
        for _ in 0..2000 {
            y = next(y);
            secrets2k.push((y%10) as i32);
        }
        secrets2klist.push(secrets2k);
    }
    let diff_codes_list = get_diff_codes(&secrets2klist);
    let (highest_value_diff_code, highest_value) = get_highest_value_diff_code(&diff_codes_list);
    println!("Highest Value Diff Code: {:?} {}", highest_value_diff_code, highest_value);
    highest_value.to_string()
}

// create a type that is an array of four i32 values
type DiffCode = [i32; 4];
// struct DiffCodeEntry {
//     dc: DiffCode,
//     price: i32,
// }

// a monkey has a Vector of 2000 secrets.
// the diference between four consectutive secrets is a DiffCode
// the price is the entry in the last secret of that set of four for the first encounter of that sequence
// get a HashMap of DiffCodeEntry from a vector of 2000 secrets
fn get_diff_codes_for_monkey(secrets: &Vec<i32>) -> HashMap<DiffCode, i32> {
    let mut diff_codes: HashMap<DiffCode, i32> = HashMap::new();
    for i in 0..1996 {
        let dc: DiffCode = [secrets[i+1] as i32 - secrets[i] as i32, secrets[i+2] as i32 - secrets[i+1] as i32, secrets[i+3] as i32 - secrets[i+2] as i32, secrets[i+4] as i32 - secrets[i+3] as i32];
        if !diff_codes.contains_key(&dc) {
            diff_codes.insert(dc, secrets[i+4]);
        }
    }
    diff_codes
}

// get the diff_codes for all the monkeys
fn get_diff_codes(secrets2klist: &Vec<Vec<i32>>) -> Vec<HashMap<DiffCode, i32>> {
    let mut diff_codes_list: Vec<HashMap<DiffCode, i32>> = Vec::new();
    for secrets2k in secrets2klist.iter() {
        diff_codes_list.push(get_diff_codes_for_monkey(secrets2k));
    }
    diff_codes_list
}

// the value of a diff code is the sum of the prices over each monkey's diff code price
fn get_diff_code_value(diff_codes_list: &Vec<HashMap<DiffCode, i32>>, diff_code: DiffCode) -> i32 {
    let mut sum = 0;
    for diff_codes in diff_codes_list.iter() {
        if let Some(price) = diff_codes.get(&diff_code) {
            sum += price;
        }
    }
    sum
}

// get a list of all the diff_codes over all the monkeys
fn get_all_diff_codes(diff_codes_list: &Vec<HashMap<DiffCode, i32>>) -> HashSet<DiffCode> {
    let mut all_diff_codes: HashSet<DiffCode> = HashSet::new();
    for diff_codes in diff_codes_list.iter() {
        for (dc, _) in diff_codes.iter() {
            all_diff_codes.insert(*dc);
        }
    }
    all_diff_codes
}

// get the diff code with the highest value
fn get_highest_value_diff_code(diff_codes_list: &Vec<HashMap<DiffCode, i32>>) -> (DiffCode, i32) {
    let all_diff_codes = get_all_diff_codes(diff_codes_list);
    let mut highest_value = 0;
    let mut highest_value_diff_code = [0, 0, 0, 0];
    for diff_code in all_diff_codes.iter() {
        let value = get_diff_code_value(diff_codes_list, *diff_code);
        if value > highest_value {
            highest_value = value;
            highest_value_diff_code = *diff_code;
        }
    }
    (highest_value_diff_code, highest_value)
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
        let answer = advent::read_file_to_string("files/test2_answer_2");
        // compare the result with the answer
        assert_eq!(result, answer);
    }
}
