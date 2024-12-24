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
    let sections = contents.split("\n\n").collect::<Vec<&str>>();
    let initial_settings = sections[0].lines().collect::<Vec<&str>>();
    let mut wires = initialize_wires(initial_settings);
    let (gates, wire_list) = set_gates(sections[1]);
    while wires.len() < wire_list.len() {
        for gate in &gates {
            apply(&mut wires, gate.0, gate.1, gate.2, gate.3);
        }
    }
    // get a Vector of the wire names that start with "z"
    let mut z_wires = Vec::new();
    for (key, value) in &wires {
        if key.starts_with("z") {
            z_wires.push(key);
        }
    }
    // sort the Vector of wire names in reverse order
    let mut answer: u128 = 0;
    z_wires.sort_by(|a, b| b.cmp(a));
    for wire in z_wires {
        answer = answer << 1 | *wires.get(wire).unwrap() as u128;
    }
    answer.to_string()
}

#[derive(Copy, Clone)]
enum Op {
    AND,
    OR,
    XOR,
}

fn apply(wires: &mut HashMap<String, bool>, in1: &str, in2: &str, op: Op, output: &str) -> Option<bool> {
    if wires.contains_key(in1) && wires.contains_key(in2) {
        let a = *wires.get(in1).unwrap();
        let b = *wires.get(in2).unwrap();
        let o = match op {
            Op::AND => Some(a && b),
            Op::OR => Some(a || b),
            Op::XOR => Some(a ^ b),
            // _ => None,
        };
        if o.is_some() {
            wires.insert(output.to_string(), o.unwrap());
        }
        return o;
    }
    return None;
}

fn set_gates(section: &str) -> (Vec<(&str, &str, Op, &str)>, HashSet<String>) {
    let mut gates = Vec::new();
    let mut wire_list = HashSet::new();
    for line in section.lines() {
        let tokens = line.split(" ").collect::<Vec<&str>>();
        wire_list.insert(tokens[0].to_string());
        wire_list.insert(tokens[2].to_string());
        wire_list.insert(tokens[4].to_string());
        match tokens[1] {
            "AND" => {
                gates.push((tokens[0], tokens[2], Op::AND, tokens[4]));
            }
            "OR" => {
                gates.push((tokens[0], tokens[2], Op::OR, tokens[4]));
            }
            "XOR" => {
                gates.push((tokens[0], tokens[2], Op::XOR, tokens[4]));
            }
            _ => {}
        }
    }
    (gates, wire_list)
}

fn initialize_wires(initial_settings: Vec<&str>) -> HashMap<String, bool> {
    let mut wires = HashMap::new();
    for setting in initial_settings {
        let parts = setting.split(": ").collect::<Vec<&str>>();
        let wire = parts[0];
        let value = parts[1].parse::<i32>().unwrap();
        wires.insert(wire.to_string(), value == 1);
    }
    wires
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
        // get the contents of the file "files/test2"
        let contents = advent::read_file_to_string("files/test2");
        // call part1 with the contents of the file
        let result = part1(&contents);
        // get the contents of the file "files/test2_answer_1"
        let answer = advent::read_file_to_string("files/test2_answer_1");
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
