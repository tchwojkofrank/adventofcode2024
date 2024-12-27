use std::{collections::{HashMap, HashSet}, time::Instant};
use itertools::Itertools;

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
    let x_value = get_value(&wires, "x".to_string());
    let y_value = get_value(&wires, "y".to_string());
    let answer = get_value(&wires, "z".to_string());
    println!("Answer: {:b} + {:b} =\n{:b}\nExpected\n{:b}", x_value, y_value, answer, x_value+y_value);
    answer.to_string()
}

fn get_value(wires: &HashMap<String, bool>, wire_prefix: String) -> u128 {
    let mut prefix_wires = Vec::new();
    for (key, _) in wires {
        if key.starts_with(wire_prefix.as_str()) {
            prefix_wires.push(key);
        }
    }
    // sort the Vector of wire names in reverse order
    let mut answer: u128 = 0;
    prefix_wires.sort_by(|a, b| b.cmp(a));
    for wire in prefix_wires {
        answer = answer << 1 | *wires.get(wire).unwrap() as u128;
    }
    answer
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
    let sections = contents.split("\n\n").collect::<Vec<&str>>();
    let initial_settings = sections[0].lines().collect::<Vec<&str>>();
    let mut wires = initialize_wires(initial_settings);
    let (mut gates, wire_list) = set_gates(sections[1]);
    while wires.len() < wire_list.len() {
        for gate in &gates {
            apply(&mut wires, gate.0, gate.1, gate.2, gate.3);
        }
    }
    // get a Vector of the wire names that start with "z"
    let x_value = get_value(&wires, "x".to_string());
    let y_value = get_value(&wires, "y".to_string());
    let z_value = get_value(&wires, "z".to_string());
    println!("Answer: {:b} + {:b} =\n{:b}\nExpected\n{:b}", x_value, y_value, z_value, x_value+y_value);
    // find which z bits don't match the expected value
    let answer = x_value + y_value;
    let mut mask: u128 = 1;
    let mut bit = 0;
    let mut wrong_bits = Vec::new();
    while mask <= answer {
        if (answer & mask) != (z_value & mask) {
            // create the name of the z gate that sets the wrong bit. The name is "z" followed by the two digit bit number
            let gate_name = format!("z{:02}", bit);
            wrong_bits.push(gate_name);
        }
        mask = mask << 1;
        bit += 1;
    }
    // get the list of gates that eventually set the wrong bits
    let mut wrong_gates = HashSet::new();
    // create a stack of the z gates, so that we can trace back to find all wires involved in gates that set the wrong bits
    let mut stack = wrong_bits.clone();
    while stack.len() > 0 {
        let gate_name = stack.pop().unwrap();
        for gate in &gates {
            if gate.3 == gate_name {
                wrong_gates.insert(gate);
                stack.push(gate.0.to_string());
                stack.push(gate.1.to_string());
            }
        }
    }
    // get the list of probably correct gates
    let mut right_bits = Vec::new();
    while mask <= answer {
        if (answer & mask) == (z_value & mask) {
            // create the name of the z gate that sets the wrong bit. The name is "z" followed by the two digit bit number
            let gate_name = format!("z{:02}", bit);
            right_bits.push(gate_name);
        }
        mask = mask << 1;
        bit += 1;
    }
    // get the list of gates that eventually set the wrong bits
    let mut right_gates = HashSet::new();
    // create a stack of the z gates, so that we can trace back to find all wires involved in gates that set the wrong bits
    let mut stack = right_bits.clone();
    while stack.len() > 0 {
        let gate_name = stack.pop().unwrap();
        for gate in &gates {
            if gate.3 == gate_name {
                right_gates.insert(gate);
                stack.push(gate.0.to_string());
                stack.push(gate.1.to_string());
            }
        }
    }
    // get the list of wrong gates that are not in the list of right gates
    let mut wrong_gates2 = HashSet::new();
    for gate in &wrong_gates {
        let mut found = false;
        for right_gate in &right_gates {
            if gate.3 == right_gate.3 {
                found = true;
                break;
            }
        }
        if !found {
            wrong_gates2.insert(gate);
        }
    }
    // print the count of wrong2 gates
    println!("Wrong2 gates: {}", wrong_gates2.len());
    // try every combination of four pairs of wrong__gates2
    // combos8 is an index into wrong_gates2
    let combos8 = (0..wrong_gates2.len()).combinations(8);
    // for each combo8, try every combination of 4 pairs of gates
    for combo8 in combos8 {
        // try every permutation of the 8 gates
        let perms = combo8.iter().permutations(8);
        for perm in perms {
        // get a reference to the first gate
        let gate0 = wrong_gates2.iter().nth(*perm[0]).unwrap().clone();
        let gate1 = wrong_gates2.iter().nth(*perm[1]).unwrap().clone();
        // find the original gates in the gates vector and swap their outputs
        for gate in gates.iter_mut() {
            if *gate == **gate0 {
                gate.3 = gate1.3;
            } else if *gate == **gate1 {
                gate.3 = gate0.3;
            }
        }
        // test if the swap results in the correct output
        let mut wires2 = wires.clone();
        for gate in &gates {
            apply(&mut wires2, gate.0, gate.1, gate.2, gate.3);
        }
        let z_value2 = get_value(&wires2, "z".to_string());
        if z_value2 == answer {
            println!("Correct answer found");
            break;
        } else {
            // undo the swap
            for gate in gates.iter_mut() {
                if *gate == **gate0 {
                    gate.3 = gate0.3;
                } else if *gate == **gate1 {
                    gate.3 = gate1.3;
                }
            }
        }
    }
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
