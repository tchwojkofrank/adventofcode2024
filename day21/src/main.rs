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

// 789
// 456
// 123
//  0A

//  ^A
// <v>

type Node = i32;

fn number_pad_distance(_x: Node, _y: Node) -> i32 {
    1
}

fn number_pad_neighbors(node: Node) -> Vec<Node> {
    match node {
        1 => vec![2, 4],
        2 => vec![0, 1, 3, 5],
        3 => vec![10, 2, 6],
        4 => vec![1, 5, 7],
        5 => vec![2, 4, 6, 8],
        6 => vec![3, 5, 9],
        7 => vec![4, 8],
        8 => vec![5, 7, 9],
        9 => vec![6, 8],
        0 => vec![2,10],
        10 => vec![0,3],
        _ => vec![],
    }
}

type Node2 = char;

fn dir_pad_distance(_x: Node2, _y: Node2) -> i32 {
    1
}

fn dir_pad_neighbors(node: Node2) -> Vec<Node2> {
    match node {
        '^' => vec!['A', 'v'],
        'A' => vec!['^', '>'],
        '<' => vec!['v'],
        'v' => vec!['<', '^', '>'],
        '>' => vec!['v', 'A'],
        _ => vec![],
    }
}

// turn off warning for unused variables
#[allow(unused_variables)]
pub fn part1(contents: &String) -> String {
    let code = vec![0,2,9,10];
    let code_paths = code.iter().map(|&node| {
        let mut path = vec![];
        let mut current = 5;
        let mut distance = 0;
        while current != node {
            let neighbors = number_pad_neighbors(current);
            let mut min_distance = std::i32::MAX;
            let mut min_node = 0;
            for &neighbor in neighbors.iter() {
                let d = number_pad_distance(neighbor, node);
                if d < min_distance {
                    min_distance = d;
                    min_node = neighbor;
                }
            }
            path.push(min_node);
            distance += min_distance;
            current = min_node;
        }
        (path, distance)
    }).collect::<Vec<_>>();
    1.to_string()
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
