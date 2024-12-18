use std::{collections::HashSet, time::Instant, sync::{LazyLock, Mutex}};

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

struct Map {
    map: HashSet<(i32, i32)>,
    width: i32,
    height: i32,
}

static MAP: LazyLock<Mutex<Map>> = LazyLock::new(|| Mutex::new(Map {
    map: HashSet::new(),
    width: 0,
    height: 0,
}));

// turn off warning for unused variables
#[allow(unused_variables)]
pub fn part1(contents: &String) -> String {
    let sections: Vec<&str> = contents.split("\n\n").collect();
    let bounds: Vec<&str> = sections[0].split(",").collect();
    let mut map = MAP.lock().unwrap();
    map.width = bounds[0].parse::<i32>().unwrap();
    map.height = bounds[1].parse::<i32>().unwrap();
    map.map = get_coordinates(&(sections[1].to_string()),1024);
    let start = Node{p: (0, 0)};
    let goal = Node{p: (map.width-1, map.height-1)};
    drop(map);
    let path = advent::shortest_path(start, goal, get_neighbors, get_distance, get_heuristic);
    // print path
    if path.is_none() {
        return "No path found".to_string();
    }
    let path = path.unwrap();
    for node in path.iter() {
        println!("{:?}", node.p);
    }
    (path.len()-1).to_string()
}
#[derive(Clone,Eq,PartialEq,Hash,Debug)]
struct Node {
    p: (i32, i32)
}

fn get_heuristic(start: &Node, end: &Node) -> u64 {
    get_distance(start, end)
}

fn get_distance(node1: &Node, node2: &Node) -> u64 {
    let x1 = node1.p.0;
    let y1 = node1.p.1;
    let x2 = node2.p.0;
    let y2 = node2.p.1;
    ((x1-x2).abs() + (y1-y2).abs()) as u64
}

fn get_neighbors(node: &Node) -> Vec<Node> {
    let mut neighbors = Vec::new();
    let map = MAP.lock().unwrap();
    let x = node.p.0;
    let y = node.p.1;
    if x > 0 {
        if !map.map.contains(&(x-1, y)) {
            neighbors.push(Node{p:(x-1, y)});
        }
    }
    if x < map.width-1 {
        if !map.map.contains(&(x+1, y)) {
            neighbors.push(Node{p:(x+1, y)});
        }
    }
    if y > 0 {
        if !map.map.contains(&(x, y-1)) {
            neighbors.push(Node{p:(x, y-1)});
        }
    }
    if y < map.height-1 {
        if !map.map.contains(&(x, y+1)) {
            neighbors.push(Node{p: (x, y+1)});
        }
    }
    drop(map);
    neighbors
}

fn get_coordinates(secton: &String,count:i32) -> HashSet<(i32,i32)> {
    let mut coordinates = HashSet::new();
    let lines = secton.split("\n");
    let mut c = 0;
    for line in lines {
        // parse the string n,n into two integers
        let parts: Vec<&str> = line.split(",").collect();
        let x = parts[0].parse::<i32>().unwrap();
        let y = parts[1].parse::<i32>().unwrap();
        coordinates.insert((x,y));
        c += 1;
        if c == count {
            break;
        }
    }
    coordinates
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
