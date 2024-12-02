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

// turn off warning for unused variables
#[allow(unused_variables)]
pub fn part1(contents: &String) -> String {
    let reports = advent::split_input_into_lines(&contents);
    let safe = count_safe_reports(&reports);
    safe.to_string()
}

fn count_safe_reports(reports: &Vec<String>) -> i32 {
    let mut count = 0;
    for report in reports {
        if is_safe_report(report) {
            count += 1;
        }
    }
    count
}

fn is_safe_report(report: &String) -> bool {
    // each report is a set of numbers separated by whitespace
    // split the report into a vector of strings
    let number_strings = advent::split_line_into_words(report);
    // convert the strings into integers
    let numbers: Vec<i32> = number_strings.iter().map(|s| s.parse().unwrap()).collect();
    // a report is safe if the numbers are either monotonically increasing or decreasing by between 1 and 3
    let mut safe = true;
    let mut sign = 0;
    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i - 1];
        if diff < 0 {
            if sign > 0 {
                safe = false;
                break;
            }
            sign = -1;
        } else if diff > 0 {
            if sign < 0 {
                safe = false;
                break;
            }
            sign = 1;
        } else {
            safe = false;
            break;
        }
        if diff.abs() < 1 || diff.abs() > 3 {
            safe = false;
            break;
        }
    }
    safe
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
