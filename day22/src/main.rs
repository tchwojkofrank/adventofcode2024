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
    let mut secrets2klist: Vec<Vec<u128>> = Vec::new();
    for secret in secrets.iter() {
        let mut secrets2k = Vec::new();
        let x = *secret;
        secrets2k.push(x);
        let mut y = x;
        for _ in 0..2000 {
            y = next(y%10);
            secrets2k.push(y);
        }
        secrets2klist.push(secrets2k);
    }
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
