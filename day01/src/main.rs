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
    let result1 = part1(&contents);
    println!("Part 1: {}", result1);
    let result2 = part2(&contents);
    println!("Part 2: {}", result2);
}

fn split_line_into_pair(line: &String) -> (i32, i32) {
    let mut pair = (0, 0);
    // the line made of a number, followed by whitespace followed by a number
    // split the line into a vector of strings, and transform the strings into integers    
    let numbers: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
    // the first number is the first element of the vector
    pair.0 = numbers[0];
    // the second number is the second element of the vector
    pair.1 = numbers[1];
    pair
}

fn get_arrays(lines: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    // create two arrays of integers, one for the first number in the line, and one for the second number in the line
    let mut firsts = Vec::new();
    let mut seconds = Vec::new();
    // iterate over the lines
    for line in lines {
        // split the line into a pair of integers
        let pair = split_line_into_pair(&line);
        // add the first number to the firsts array
        firsts.push(pair.0);
        // add the second number to the seconds array
        seconds.push(pair.1);
    }
    (firsts, seconds)
}

fn array_differences(firsts: &Vec<i32>, seconds: &Vec<i32>) -> Vec<i32> {
    // get the absolute value of the differences of each pair of numbers between the two arrays
    let mut differences = Vec::new();
    for i in 0..firsts.len() {
        differences.push((firsts[i] - seconds[i]).abs());
    }
    differences
}

#[allow(unused_variables)]
pub fn part1(contents: &String) -> String {
    let lines = advent::split_input_into_lines(&contents);
    // create two arrays of integers, one for the first number in the line, and one for the second number in the line
    let (firsts, seconds) = get_arrays(&lines);
    // sort both arrays
    let mut firsts = firsts;
    firsts.sort();
    let mut seconds = seconds;
    seconds.sort();
    // get the differences of each pair of numbers between the two arrays
    let differences = array_differences(&firsts, &seconds);
    // get the sum of the differences
    let sum: i32 = differences.iter().sum();
    sum.to_string()
}

#[allow(unused_variables)]
pub fn part2(contents: &String) -> String {
    let lines = advent::split_input_into_lines(&contents);
    // create two arrays of integers, one for the first number in the line, and one for the second number in the line
    let (lefts, rights) = get_arrays(&lines);

    // create a hashmap for the number of times each number appears in the right array
    let mut right_map = std::collections::HashMap::new();
    for right in rights {
        let count = right_map.entry(right).or_insert(0);
        *count += 1;
    }

    // for each number N in the left array, get the number of times it appears in the right array and multiply it by N, and sum the results
    let mut sum = 0;
    for left in lefts {
        let count = right_map.get(&left).unwrap_or(&0);
        sum += left * count;
    }

    sum.to_string()
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
