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

#[derive(Clone)]
struct Machine {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u64>,
    op: usize,
    output: Vec<u64>,
    check: bool,
}

fn make_machine(contents: &String) -> Machine {
    let sections = contents.split("\n\n").collect::<Vec<&str>>();
    let program = sections[1].strip_prefix("Program: ").unwrap().trim();
    // strip off the prefix of "Register A: " what's left is the value of register A
    let lines = sections[0].lines().collect::<Vec<&str>>();
    let machine = Machine {
        a: lines[0].strip_prefix("Register A: ").unwrap().parse::<u64>().unwrap(),
        b: lines[1].strip_prefix("Register B: ").unwrap().parse::<u64>().unwrap(),
        c: lines[2].strip_prefix("Register C: ").unwrap().parse::<u64>().unwrap(),
        // program is a comma separated list of numbers
        program: program.split(",").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>(),
        op: 0,
        check: false,
        output: Vec::new(),
    };
    machine
}

// define the operation functions on a Machine
impl Machine {
    #[allow(non_snake_case)]
    fn AdivideOp(&mut self) {
        self.a = self.a >> self.Op();
        self.op += 2;
    }

    #[allow(non_snake_case)]
    fn BxorLit(&mut self) {
        self.b = self.b ^ self.program[self.op+1];
        self.op += 2;
    }

    #[allow(non_snake_case)]
    fn BstoreOp(&mut self) {
        self.b = self.Op() & 7;
        self.op += 2;
    }

    #[allow(non_snake_case)]
    fn Jnz(&mut self) {
        if self.a != 0 {
            self.op = self.program[self.op+1] as usize;
        } else {
            self.op += 2;
        }
    }

    #[allow(non_snake_case)]
    fn BxorC(&mut self) {
        self.b ^= self.c;
        self.op += 2;
    }

    #[allow(non_snake_case)]
    fn Out(&mut self) -> bool {
        // append ",{operand}" to the output string
        self.output.push(self.Op() & 7);
        self.op += 2;
        if self.check {
            if self.output[self.output.len()-1] != self.program[self.output.len()-1] {
                return false;
            }
        }
        true
    }

    #[allow(non_snake_case)]
    fn BdivOp(&mut self) {
        self.b = self.a >> self.Op();
        self.op += 2;
    }

    #[allow(non_snake_case)]
    fn CdivOp(&mut self) {
        self.c = self.a >> self.Op();
        self.op += 2;
    }

    #[allow(non_snake_case)]
    fn Op(&self) -> u64{
        let value = self.program[self.op+1];
        if value <= 3 {
            return value;
        }
        match value {
            4 => return self.a,
            5 => return self.b,
            6 => return self.c,
            _ => panic!("Invalid opcode")
        }
    }

    #[allow(non_snake_case)]
    fn Run(&mut self) -> (bool, bool) {
        let mut ok = true;
        let mut ok_prefix = true;
        while ok && self.op < self.program.len() {
            match self.program[self.op] {
                0 => self.AdivideOp(),
                1 => self.BxorLit(),
                2 => self.BstoreOp(),
                3 => self.Jnz(),
                4 => self.BxorC(),
                5 => ok = self.Out(),
                6 => self.BdivOp(),
                7 => self.CdivOp(),
                _ => panic!("Invalid opcode")
            }
        }
        if self.check {
            // check that the output is the same as the program
            if self.output.len() != self.program.len() {
                ok = false;
            }
            for i in 0..self.output.len() {
                if self.output[i] != self.program[i] {
                    if i == self.output.len()-1 && i > 0 {
                        ok_prefix = true;
                    } else {
                        ok_prefix = false;
                    }
                    ok = false
                }
            }
        }
        (ok, ok_prefix)
    }
}

// turn off warning for unused variables
#[allow(unused_variables)]
pub fn part1(contents: &String) -> String {
    let mut machine = make_machine(contents);
    machine.Run();
    machine.output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
}

#[allow(unused_variables)]
pub fn part2(contents: &String) -> String {
    let mut result = "0".to_string();
    let mut result_value = 0 as u64;
    let original_machine = make_machine(contents);
    let mut ok = (false,false);
    let mut longest_output = String::new();
    let mut best_suffix = "".to_string();
    let mut answer = 0;
    while !ok.0 {
        // println!("Trying {}", result);
        let mut machine = original_machine.clone();
        machine.check = true;
        machine.a = u64::from_str_radix(&(result.clone() + &best_suffix), 2).unwrap();
        answer = machine.a;
        println!("Trying {}. Longest match {}", machine.a, longest_output);
        ok = machine.Run();
        let mut machine_output = machine.output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
        // machine_output may have 0 to n commas. Trim everything after the last comma
        machine_output = machine_output.rsplit_once(',').map_or(machine_output.clone(), |(prefix, _)| prefix.to_string());
        let mut reset_result = false;
        if machine_output.len() > longest_output.len() && ok.1 {
            // trim everything after the last comma
            longest_output = machine_output.clone();
            best_suffix = result.clone() + &best_suffix;
            println!("{}: {}", best_suffix, longest_output);
            reset_result = true;
        }
        if !ok.0 {
            if reset_result {
                result_value = 0;
            } else {
                result_value += 1;
            }
            result = format!("{:b}", result_value);
        }
    }

    answer.to_string()
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
        let answer = advent::read_file_to_string("files/test_answer_2");
        // compare the result with the answer
        assert_eq!(result, answer);
    }
}
