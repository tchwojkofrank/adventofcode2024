
// use the advent package
use advent;
use std::{collections::{HashMap, HashSet}, sync::Mutex, time::Instant, io::Write};

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
    let map = make_map(contents);
    // assign map to the global variable MAP
    let mut m = MAP.lock().unwrap();
    *m = Some(map.clone());
    // unlock MAP
    drop(m);
    let start_pos = map.1;
    let end_pos = map.2;
    let start_node = Node { pos: start_pos, direction: 1 };
    let end_node = Node { pos: end_pos, direction: 4 };
    // clear the screen
    advent::clear();
    advent::position(0,0);
    // print the map
    // set the color to white
    print!("\x1b[37m");

    for row in map.0 {
        let s: String = row.iter().collect();
        println!("{}", s);
    }
    let path = advent::shortest_path_with_callback(start_node, end_node, get_neighbors, get_cost, heuristic, Some(visit));
    if path.is_none() {
        return "No path found".to_string();
    }
    let path_cost = calculate_path_cost(&path.unwrap());
    path_cost.to_string()
}

fn calculate_path_cost(path: &Vec<Node>) -> u64 {
    let mut cost = 0;
    for i in 0..path.len()-1 {
        let new_cost = get_cost(&path[i], &path[i+1]);
        // print the node in green
        advent::position(path[i].pos.0 as u32, path[i].pos.1 as u32);
        print!("\x1b[32m■\x1b[0m");
        // println!("Cost from {:?} to {:?} is {}", path[i], path[i+1], new_cost);
        cost += new_cost;
    }
    // move the cursor to the bottom of the grid
    advent::position(0, MAP.lock().unwrap().as_ref().unwrap().0.len() as u32);
    cost
}
// direction: 0 = up, 1 = right, 2 = down, 3 = left
#[derive(Clone,Eq,PartialEq,Hash,Debug)]
struct Node {
    pos: (i32, i32),
    direction: i32
}

fn get_neighbors(node: &Node) -> Vec<Node> {
    let mut neighbors = Vec::new();
    // get a reference to the global variable MAP
    let m = MAP.lock().unwrap();
    let map = m.as_ref().unwrap();
    // the neighbors are up, right, down, left, if they are not a wall, and the cost is:
    // 1 if the direction is the same as the current direction
    // 1001 if the direction is 90 degrees from the current direction
    // 2001 if the direction is 180 degrees from the current direction
    let directions = vec![(0,-1), (1,0), (0,1), (-1,0)];
    for (i, (dx, dy)) in directions.iter().enumerate() {
        // check if the direction is the opposite of the current direction
        if (node.direction - i as i32).abs() == 2 {
            continue;
        }
        if i != node.direction as usize {
            if map.0[(node.pos.1+dy) as usize][(node.pos.0+dx) as usize] != '#' {
                neighbors.push(Node { pos: node.pos, direction: i as i32 });
            }
            continue;
        }
        let x = node.pos.0 + dx;
        let y = node.pos.1 + dy;
        if x >= 0 && x < map.0[0].len() as i32 && y >= 0 && y < map.0.len() as i32 {
            let c = map.0[y as usize][x as usize];
            if c == 'E' {
                neighbors.push(Node { pos: (x, y), direction: 4 });
            }
            if c != '#' {
                neighbors.push(Node { pos: (x, y), direction: i as i32 });
            }
        }
    }
    // unlock MAP
    drop(m);
    neighbors
}

fn get_cost(node: &Node, neighbor: &Node) -> u64 {
    if node.direction != neighbor.direction && neighbor.direction != 4 {
        let diff = (node.direction - neighbor.direction).abs();
        if diff == 2 {
            return 2000;
        } else {
            return 1000;
        }
    }
    1
}

fn heuristic(_node1: &Node, _node2: &Node) -> u64 {
    // let dx = (node1.pos.0 - node2.pos.0).abs();
    // let dy = (node1.pos.1 - node2.pos.1).abs();
    // (dx + dy) as u64
    0
}

lazy_static::lazy_static! {
    static ref MAP: Mutex<Option<(Vec<Vec<char>>,(i32,i32),(i32,i32))>> = Mutex::new(None);
}

fn visit(node: &Node) {
    // print a red dot at the node's position using the advent package
    advent::position(node.pos.0 as u32, node.pos.1 as u32);
    print!("\x1b[31m•\x1b[0m");
    std::io::stdout().flush().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(1));
}

fn make_map(contents: &String) -> (Vec<Vec<char>>, (i32, i32), (i32, i32)) {
    let mut map = Vec::new();
    let mut start: (i32, i32) = (0,0);
    let mut end: (i32, i32) = (0,0);
    for (y,line) in contents.lines().enumerate() {
        let mut row = Vec::new();
        for (x,c) in line.chars().enumerate() {
            row.push(c);
            match c {
                'S' => {
                    start = (x as i32, y as i32);
                },
                'E' => {
                    end = (x as i32, y as i32);
                },
                _ => {}
            }
        }
        map.push(row);
    }
    (map, start, end)
}

#[allow(unused_variables)]
pub fn part2(contents: &String) -> String {
    let map = make_map(contents);
    // assign map to the global variable MAP
    let mut m = MAP.lock().unwrap();
    *m = Some(map.clone());
    // unlock MAP
    drop(m);
    let start_pos = map.1;
    let end_pos = map.2;
    let start_node = Node { pos: start_pos, direction: 1 };
    let end_node = Node { pos: end_pos, direction: 4 };
    let paths = advent::all_shortest_paths(start_node, end_node, get_neighbors, get_cost, heuristic);
    if paths.is_none() {
        return "No path found".to_string();
    }
    let visited = walk_paths(paths.unwrap());
    visited.len().to_string()
}

fn walk_paths(paths: HashMap<Node,Vec<Node>>) -> HashSet<(i32,i32)> {
    let mut visited = HashSet::new();
    let mut visited_stack = HashSet::new();
    // start at the end node from the map, and walk backwards on all paths in the HashMap that lead back to the start.
    // add each node to the visited HashSet
    // return the HashSet
    // get the global MAP
    let m = MAP.lock().unwrap();
    let map = m.as_ref().unwrap();
    let end = Node { pos: map.2, direction: 4 };
    let start = Node { pos: map.1, direction: 1 };
    let mut result_map = map.0.clone();
    // unlock MAP
    drop(m);
    // walk the paths backwards starting from the end node
    let mut stack = vec![end];
    while stack.len() > 0 {
        let node = stack.pop().unwrap();
        if visited_stack.contains(&node) {
            continue;
        }
        visited.insert(node.pos.clone());
        visited_stack.insert(node.clone());
        result_map[node.pos.1 as usize][node.pos.0 as usize] = 'O';
        if node.pos == start.pos {
            continue;
        }
        let neighbors = paths.get(&node).unwrap();
        for neighbor in neighbors {
            stack.push(neighbor.clone());
        }
    }
    // print the map
    // for row in result_map {
    //     let s: String = row.iter().collect();
    //     println!("{}", s);
    // }
    visited
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
