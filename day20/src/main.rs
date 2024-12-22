use std::{collections::HashMap, sync::{LazyLock, Mutex}, time::Instant};

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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct Node {
    x: i32,
    y: i32,
}

struct Map {
    map: HashMap<(i32, i32),char>,
    start: (i32,i32),
    end: (i32,i32),
    width: i32,
    height: i32,
}

static MAP: LazyLock<Mutex<Map>> = LazyLock::new(|| Mutex::new(Map {
    map: HashMap::new(),
    start: (0,0),
    end: (0,0),
    width: 0,
    height: 0,
}));


fn get_neighbors(node: &Node) -> Vec<Node> {
    // get the global map
    let m = MAP.lock().unwrap();
    let mut neighbors = Vec::new();
    let mut x = node.x+1;
    let mut y = node.y;
    if x < m.width-1 {
        let grid_value = m.map.get(&(x,y)).unwrap_or(&'.');
        if *grid_value != '#'{
            neighbors.push(Node{x: x, y:y});
        }
    }
    x = node.x-1;
    if x > 0 {
        let grid_value = m.map.get(&(x,y)).unwrap_or(&'.');
        if *grid_value != '#'{
            neighbors.push(Node{x: x, y:y,});
        }
    }
    x = node.x;
    y = node.y+1;
    if y < m.height-1 {
        let grid_value = m.map.get(&(x,y)).unwrap_or(&'.');
        if *grid_value != '#'{
            neighbors.push(Node{x: x, y:y});
        }
    }
    y = node.y-1;
    if y > 0 {
        let grid_value = m.map.get(&(x,y)).unwrap_or(&'.');
        if *grid_value != '#'{
            neighbors.push(Node{x: x, y:y});
        }
    }
    drop(m);
    neighbors
}

fn get_heuristic(start: &Node, end: &Node) -> u64 {
    return ((start.x - end.x).abs() + (start.y - end.y).abs()) as u64;
}

fn get_distance(_node1: &Node, _node2: &Node) -> u64 {
    1
}

fn make_map(contents: &String) {
    let mut m = MAP.lock().unwrap();
    let mut x;
    let mut y = 0;
    let lines = contents.split("\n").collect::<Vec<&str>>();
    m.width = lines.len() as i32;
    m.height = lines[0].len() as i32;
    for line in lines {
        x = 0;
        for c in line.chars() {
            if c == '#' {
                m.map.insert((x,y), c);
            }
            if c == 'S' {
                m.start = (x,y);
            }
            if c == 'E' {
                m.end = (x,y);
            }
            x += 1;
        }
        y += 1;
    }
    drop(m);
}

// turn off warning for unused variables
#[allow(unused_variables)]
pub fn part1(contents: &String) -> String {
    make_map(contents);
    // get the shortest path without cheating (by setting the cheated flag to true)
    let m = MAP.lock().unwrap();
    let start = Node{x: m.start.0, y: m.start.1};
    let end = Node{x: m.end.0, y: m.end.1};
    drop(m);
    let best = advent::shortest_path(start, end , get_neighbors, get_distance, get_heuristic);
    if best.is_none() {
        return "No path found".to_string();
    }
    let best = best.unwrap();
    let best_distance = best.len()-1;
    let mut path_index: HashMap<(i32,i32),i32> = HashMap::new();
    for (i,node) in best.iter().enumerate() {
        path_index.insert((node.x,node.y), i as i32);
    }
    let mut cheats: HashMap<(i32,i32),i32> = HashMap::new();
    for (i,node) in best.iter().enumerate() {
        // check each direction for a possible cheat by going through a wall
        let x = node.x;
        let y = node.y;
        let directions = vec![(0,1),(0,-1),(1,0),(-1,0)];
        for (dx,dy) in directions {
            let x1 = x+dx;
            let y1 = y+dy;
            let x2 = x+2*dx;
            let y2 = y+2*dy;
            let m = MAP.lock().unwrap();
            let c1 = *(m.map.get(&(x1,y1)).unwrap_or(&'.'));
            let c2 = *(m.map.get(&(x2,y2)).unwrap_or(&'.'));
            drop(m);
            // make sure the next position is a wall, and the position after that is not a wall
            if c1 == '#' && c2 != '#' {
                if path_index.get(&(x2,y2)).is_some() {
                    let pi1 = path_index.get(&(x,y)).unwrap();
                    let pi2 = path_index.get(&(x2,y2)).unwrap();
                    if pi1 - pi2 > 100 {
                        cheats.insert((x1,y1), pi2-pi1-2);
                    }
                }
            }
        }
    }

    // count how many cheats save more than 100 steps
    cheats.len().to_string()
}

#[allow(unused_variables)]
pub fn part2(contents: &String) -> String {
    make_map(contents);
    // get the shortest path without cheating (by setting the cheated flag to true)
    let m = MAP.lock().unwrap();
    let start = Node{x: m.start.0, y: m.start.1};
    let end = Node{x: m.end.0, y: m.end.1};
    drop(m);
    let best = advent::shortest_path(start, end , get_neighbors, get_distance, get_heuristic);
    if best.is_none() {
        return "No path found".to_string();
    }
    let best = best.unwrap();
    let best_distance = best.len()-1;
    let mut path_index: HashMap<(i32,i32),i32> = HashMap::new();
    for (i,node) in best.iter().enumerate() {
        path_index.insert((node.x,node.y), i as i32);
    }
    let mut cheats: HashMap<(i32,i32),i32> = HashMap::new();
    let mut cheat_counts: HashMap<i32,i32> = HashMap::new();

    let m = MAP.lock().unwrap();
    let width = m.width;
    let height = m.height;
    drop(m);

    // start at the beginning of the path, and look at all grid points at most 20 manhattan distance away
    for (i,node) in best.iter().enumerate() {
        // println!("Checking node {} of {}", i, best.len());  
        let x = node.x;
        let y = node.y;
        for x1 in x-20..=x+20 {
            for y1 in y-20..=y+20 {
                if x1 < 0 || y1 < 0 || x1 >= width || y1 >= height || (x==x1 && y==y1) {
                    continue;
                }
                // now check to see if the distance saved is more than 100
                // the distance saved is the index of the end point minus the index of the start point, - the manhattan distance between the start and end points
                if path_index.get(&(x1,y1)).is_some() {
                    let pi1 = path_index.get(&(x,y)).unwrap();
                    let pi2 = path_index.get(&(x1,y1)).unwrap();
                    // make sure the end point is further along the path than the start point
                    if pi2 <= pi1 {
                        continue;
                    }
                    let manhattan = (x1-x).abs() + (y1-y).abs();
                    if manhattan > 20 {
                        continue;
                    }
                    if (pi2-pi1-manhattan) >= 100 {
                        let cheat_saving = pi2-pi1-manhattan;
                        let count = cheat_counts.entry(cheat_saving).or_insert(0);
                        *count += 1;
                    }
                    if pi2 - pi1 - manhattan >= 100 {
                        cheats.insert((x1,y1), pi2-pi1-manhattan);
                    }
                }
            }
        }
    }

    let mut cc_sum = 0;
    for (k,v) in cheat_counts.iter() {
        if *k >= 100 {
            cc_sum += *v;
        }
    }
    println!("Total cheats >= 100: {}", cc_sum);

    cc_sum.to_string()
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
