use std::time::Instant;
use regex::Regex;

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
    let machine_sections = contents.split("\n\n").collect::<Vec<&str>>();
    let machines = machine_sections.iter().map(|section| get_machine(section)).collect::<Vec<Machine>>();
    let mut tokens = 0;
    for machine in machines {
        match win_prize(&machine) {
            Ok((na,nb)) => {
                tokens += 3 * na + nb;
            },
            Err(_) => {}
        }
    }

    tokens.to_string()
}

struct Machine {
    a: (i128,i128),
    b: (i128,i128),
    p: (i128,i128),
}

fn win_prize(m: &Machine) -> Result<(i128,i128),&str> {
    // figure out how many times to press button A and button B to reach the prize location
    // the prize location is at m.p
    // the location starts at (0,0)
    // the location is updated by pressing button A or button B, which moves the location by the corresponding amount
    // if na is the number of times button A is pressed and nb is the number of times button B is pressed, then
    // the location is updated by (na * m.a.0 + nb * m.b.0, na * m.a.1 + nb * m.b.1)
    // na * m.a.0 + nb * m.b.0 = m.p.0
    // na * m.a.1 + nb * m.b.1 = m.p.1
    // solve for na and nb
    // na = (m.p.0 * m.b.1 - m.p.1 * m.b.0) / (m.a.0 * m.b.1 - m.a.1 * m.b.0)
    // nb = (m.p.1 * m.a.0 - m.p.0 * m.a.1) / (m.a.0 * m.b.1 - m.a.1 * m.b.0)
    // if na and nb are both positive integers, then the prize can be won
    // if na and nb are not both positive integers, then the prize cannot be won
    let na = (m.p.0 * m.b.1 - m.p.1 * m.b.0) / (m.a.0 * m.b.1 - m.a.1 * m.b.0);
    let nar = (m.p.0 * m.b.1 - m.p.1 * m.b.0) % (m.a.0 * m.b.1 - m.a.1 * m.b.0);
    let nb = (m.p.1 * m.a.0 - m.p.0 * m.a.1) / (m.a.0 * m.b.1 - m.a.1 * m.b.0);
    let nbr = (m.p.1 * m.a.0 - m.p.0 * m.a.1) % (m.a.0 * m.b.1 - m.a.1 * m.b.0);
    if na > 0 && nb > 0 && na <= 100 && nb <= 100 && nar == 0 && nbr == 0 {
        Ok((na,nb))
    } else {
        Err("Cannot win prize")
    }
}

fn win_prize2(m: &Machine) -> Result<(i128,i128),&str> {
    // figure out how many times to press button A and button B to reach the prize location
    // the prize location is at m.p
    // the location starts at (0,0)
    // the location is updated by pressing button A or button B, which moves the location by the corresponding amount
    // if na is the number of times button A is pressed and nb is the number of times button B is pressed, then
    // the location is updated by (na * m.a.0 + nb * m.b.0, na * m.a.1 + nb * m.b.1)
    // na * m.a.0 + nb * m.b.0 = m.p.0
    // na * m.a.1 + nb * m.b.1 = m.p.1
    // solve for na and nb
    // na = (m.p.0 * m.b.1 - m.p.1 * m.b.0) / (m.a.0 * m.b.1 - m.a.1 * m.b.0)
    // nb = (m.p.1 * m.a.0 - m.p.0 * m.a.1) / (m.a.0 * m.b.1 - m.a.1 * m.b.0)
    // if na and nb are both positive integers, then the prize can be won
    // if na and nb are not both positive integers, then the prize cannot be won
    let m = Machine {a: m.a, b: m.b, p: (m.p.0+10_000_000_000_000, m.p.1+10_000_000_000_000)};
    let na = (m.p.0 * m.b.1 - m.p.1 * m.b.0) / (m.a.0 * m.b.1 - m.a.1 * m.b.0);
    let nar = (m.p.0 * m.b.1 - m.p.1 * m.b.0) % (m.a.0 * m.b.1 - m.a.1 * m.b.0);
    let nb = (m.p.1 * m.a.0 - m.p.0 * m.a.1) / (m.a.0 * m.b.1 - m.a.1 * m.b.0);
    let nbr = (m.p.1 * m.a.0 - m.p.0 * m.a.1) % (m.a.0 * m.b.1 - m.a.1 * m.b.0);
    if na > 0 && nb > 0 && nar == 0 && nbr == 0 {
        Ok((na,nb))
    } else {
        Err("Cannot win prize")
    }
}

fn get_machine(machine_section: &str) -> Machine {
    // the string is of the form:
    // Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: 8400, 5400
    let re: Regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();
    let caps = re.captures(machine_section).unwrap();
    let a = (caps[1].parse::<i128>().unwrap(), caps[2].parse::<i128>().unwrap());
    let b = (caps[3].parse::<i128>().unwrap(), caps[4].parse::<i128>().unwrap());
    let p = (caps[5].parse::<i128>().unwrap(), caps[6].parse::<i128>().unwrap());
    Machine {a,b,p}
}

#[allow(unused_variables)]
pub fn part2(contents: &String) -> String {
    let machine_sections = contents.split("\n\n").collect::<Vec<&str>>();
    let machines = machine_sections.iter().map(|section| get_machine(section)).collect::<Vec<Machine>>();
    let mut tokens = 0;
    for machine in machines {
        match win_prize2(&machine) {
            Ok((na,nb)) => {
                tokens += 3 * na + nb;
            },
            Err(_) => {}
        }
    }

    tokens.to_string()
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
