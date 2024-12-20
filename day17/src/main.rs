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
    println!("Part 1:\n{}\n\tTook {:?}\n", result1, duration);

    // println!("Experimenting:\n{}\n", experiment(&contents));

    let start = Instant::now();
    let result2 = part2(&contents);
    let duration = start.elapsed();
    println!("Part 2:\n{}\n\tTook {:?}", result2, duration);
}

#[derive(Clone)]
struct Machine {
    a: u128,
    b: u128,
    c: u128,
    program: Vec<u128>,
    op: usize,
    output: Vec<u128>,
    check: bool,
}

fn make_machine(contents: &String) -> Machine {
    let sections = contents.split("\n\n").collect::<Vec<&str>>();
    let program = sections[1].strip_prefix("Program: ").unwrap().trim();
    // strip off the prefix of "Register A: " what's left is the value of register A
    let lines = sections[0].lines().collect::<Vec<&str>>();
    let machine = Machine {
        a: lines[0].strip_prefix("Register A: ").unwrap().parse::<u128>().unwrap(),
        b: lines[1].strip_prefix("Register B: ").unwrap().parse::<u128>().unwrap(),
        c: lines[2].strip_prefix("Register C: ").unwrap().parse::<u128>().unwrap(),
        // program is a comma separated list of numbers
        program: program.split(",").map(|x| x.parse::<u128>().unwrap()).collect::<Vec<u128>>(),
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
    fn Op(&self) -> u128{
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

pub fn experiment(contents: &String) -> String {
    let mut machine = make_machine(contents);
    machine.Run();
    let machine_output = machine.output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
    machine_output
}

#[allow(unused_variables)]
pub fn part2(contents: &String) -> String {
    let machine = make_machine(contents);
    let (_,a) = test_target2(&machine, 0, 0);
    a.to_string()
}

fn bit_length(x: u128) -> u32 {
    let mut length = 0;
    let mut value = x;
    while value > 0 {
        value >>= 1;
        length += 1;
    }
    if length == 0 {
        return 1;
    }
    length
}

fn test_target2(machine: &Machine, target_index: usize, prefix: u128) -> (bool, u128) {
    let mut suffix = 0;
    loop {
        if bit_length(suffix) > 9 {
            return (false, 0);
        }
        let mut test_machine = machine.clone();
        let check_length = target_index+1;
        let program_start = 0;
        test_machine.a = (prefix << bit_length(suffix)) | suffix;
        println!("Testing: {:b} = {}", test_machine.a, test_machine.a);
        if bit_length(test_machine.a) > 3* (machine.program.len() as u32)+6 || bit_length(test_machine.a) > (check_length as u32)*3+6 {
            return (false, 0);
        }
        test_machine.Run();
        let mut ok = true;
        if test_machine.output.len() < check_length || test_machine.output.len() > machine.program.len() {
            ok = false;
        } else {
            for i in 0..check_length {
                if test_machine.output[i] != machine.program[i] {
                    ok = false;
                    break;
                }
            }    
        }
        if ok {
            if target_index == 0 && test_machine.output.len() == machine.program.len() {
                return (ok, (prefix << bit_length(suffix)) | suffix);
            } else {
                let (ok, a) = test_target2(machine, target_index+1, (prefix << bit_length(suffix)) | suffix);
                if ok {
                    return (ok, a);
                }
            }
        }
        suffix += 1;
    }
}

fn test_target(machine: &Machine, target_index: usize, prefix: u128) -> (bool, u128) {
    let mut suffix = 0;
    loop {
        if bit_length(suffix) > 9 {
            return (false, 0);
        }
        let mut test_machine = machine.clone();
        let check_length = machine.program.len()-target_index;
        let program_start = machine.program.len()-check_length;
        test_machine.a = (prefix << bit_length(suffix)) | suffix;
        if bit_length(test_machine.a) > 3* (machine.program.len() as u32)+9 || bit_length(test_machine.a) > (check_length as u32)*3+9 {
            return (false, 0);
        }
        println!("Testing: {:b} = {}", test_machine.a, test_machine.a);
        test_machine.Run();
        let mut ok = true;
        if test_machine.output.len() < check_length || test_machine.output.len() > machine.program.len() {
            ok = false;
        } else {
            let output_start = test_machine.output.len()-check_length;
            for i in 0..check_length {
                let program_index = program_start+i;
                let output_index = output_start+i;
                if test_machine.output[output_index] != machine.program[program_index] {
                    ok = false;
                    break;
                }
            }    
        }
        if ok {
            if target_index == 0 && test_machine.output.len() == machine.program.len() {
                return (ok, (prefix << bit_length(suffix)) | suffix);
            } else {
                let (ok, a) = test_target(machine, target_index-1, (prefix << bit_length(suffix)) | suffix);
                if ok {
                    return (ok, a);
                }
            }
        }
        suffix += 1;
    }
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
