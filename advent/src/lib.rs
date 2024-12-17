use std::collections::HashMap;
use num::traits::Signed;
use std::cmp::Reverse;


pub fn get_commandline_arguments() -> Vec<String> {
    // if there are no arguments, return an empty vector
    if std::env::args().count() == 1 {
        return Vec::new();
    }
    let mut args = Vec::new();
    for arg in std::env::args().skip(1) {
        args.push(arg);
    }
    args
}

pub fn read_file_to_string(filename: &str) -> String {
    std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
}

// Break up the input into sections, which are separated by blank lines
pub fn split_input_into_sections(input: &str) -> Vec<String> {
    input.split("\n\n")
        .map(|s| s.to_string())
        .collect()
}

// Break up the input into lines
pub fn split_input_into_lines(input: &String) -> Vec<String> {
    input.lines()
        .map(|s| s.to_string())
        .collect()
}

// Break up a line into words
pub fn split_line_into_words(line: &String) -> Vec<String> {
    line.split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

// Djikstra's shortest path algorithm abstraction
// Requires an abstract node type,
// a function to get neighbors of a node,
// a function to get the distance between two nodes,
// and a function to get the heuristic distance between two nodes
#[derive(Debug, Clone, Eq, Hash)]
struct PriorityNode<Node> {
    priority: u64,
    node: Node,
}

impl<Node> Ord for PriorityNode<Node> 
where Node: std::cmp::Eq + std::hash::Hash + std::clone::Clone
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

impl<Node> PartialOrd for PriorityNode<Node> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

impl<Node> PartialEq for PriorityNode<Node> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

pub fn shortest_path<Node, NeighborFn, DistanceFn, HeuristicFn>(start: Node, goal: Node, get_neighbors: NeighborFn, get_distance: DistanceFn, get_heuristic: HeuristicFn) -> Option<Vec<Node>>
where
    Node: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
    NeighborFn: Fn(&Node) -> Vec<Node>,
    DistanceFn: Fn(&Node, &Node) -> u64,
    HeuristicFn: Fn(&Node, &Node) -> u64,
{
    // create a priority queue
    let mut queue = std::collections::BinaryHeap::new();
    // create a hash map to store the distance to each node
    let mut distance = std::collections::HashMap::new();
    // create a hash map to store the previous node in the path
    let mut previous: HashMap<Node,Node> = std::collections::HashMap::new();
    // create a hash set to store the nodes that have been visited
    let mut visited = std::collections::HashSet::new();
    // insert the start node into the queue with a distance of zero
    queue.push(Reverse(PriorityNode { priority: 0, node: start.clone() }));
    // set the distance to the start node to zero
    distance.insert(start.clone(), 0);
    // while the queue is not empty
    while let Some(Reverse(PriorityNode { priority: dist, node })) = queue.pop() {
        // if the node is the goal node
        if node == goal {
            // create a vector to store the path
            let mut path = Vec::new();
            // set the current node to the goal node
            let mut current = goal.clone();
            // while the current node is not the start node
            while current != start {
                // insert the current node into the path vector
                path.push(current.clone());
                // set the current node to the previous node
                current = previous[&current].clone();
            }
            // insert the start node into the path vector
            path.push(start.clone());
            // reverse the path vector
            path.reverse();
            // return the path vector
            return Some(path);
        }
        // if the node has been visited
        if visited.contains(&node) {
            // skip the rest of the loop
            continue;
        }
        // insert the node into the visited set
        visited.insert(node.clone());
        // for each neighbor of the node
        for neighbor
        in get_neighbors(&node) {
            // calculate the new distance to the neighbor
            let new_distance = dist + get_distance(&node, &neighbor);
            // if the neighbor has not been visited or the new distance is less than the current distance
            if !visited.contains(&neighbor) || new_distance < *distance.get(&neighbor).unwrap_or(&u64::MAX) {
                // insert the new distance into the distance hash map
                distance.insert(neighbor.clone(), new_distance);
                // insert the neighbor into the queue with the new distance
                queue.push(Reverse(PriorityNode { priority: new_distance + get_heuristic(&neighbor, &goal), node: neighbor.clone() }));
                // insert the node into the previous hash map
                previous.insert(neighbor.clone(), node.clone());
            }
        }
    }
    // return None if no path is found
    None
}

pub fn all_shortest_paths<Node, NeighborFn, DistanceFn, HeuristicFn>(start: Node, goal: Node, get_neighbors: NeighborFn, get_distance: DistanceFn, get_heuristic: HeuristicFn) -> Option<HashMap<Node,Vec<Node>>>
where
    Node: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
    NeighborFn: Fn(&Node) -> Vec<Node>,
    DistanceFn: Fn(&Node, &Node) -> u64,
    HeuristicFn: Fn(&Node, &Node) -> u64,
{
    // create a priority queue
    let mut queue = std::collections::BinaryHeap::new();
    // create a hash map to store the distance to each node
    let mut distance = std::collections::HashMap::new();
    // create a hash map to store the previous node in the path
    let mut previous: HashMap<Node,Vec<Node>> = std::collections::HashMap::new();
    // create a hash set to store the nodes that have been visited
    let mut visited = std::collections::HashSet::new();
    // insert the start node into the queue with a distance of zero
    queue.push(Reverse(PriorityNode { priority: 0, node: start.clone() }));
    // set the distance to the start node to zero
    distance.insert(start.clone(), 0);
    let mut found_path = false;
    // while the queue is not empty
    while let Some(Reverse(PriorityNode { priority: dist, node })) = queue.pop() {
        // if the node is the goal node
        if node == goal {
            found_path = true;
        }
        // if the node has been visited
        if visited.contains(&node) {
            // skip the rest of the loop
            continue;
        }
        // insert the node into the visited set
        visited.insert(node.clone());
        // for each neighbor of the node
        for neighbor
        in get_neighbors(&node) {
            // calculate the new distance to the neighbor
            let new_distance = dist + get_distance(&node, &neighbor);
            // if the neighbor has not been visited or the new distance is less than the current distance
            if !visited.contains(&neighbor) || new_distance <= *distance.get(&neighbor).unwrap_or(&u64::MAX) {
                // insert the new distance into the distance hash map
                distance.insert(neighbor.clone(), new_distance);
                // insert the neighbor into the queue with the new distance
                queue.push(Reverse(PriorityNode { priority: new_distance + get_heuristic(&neighbor, &goal), node: neighbor.clone() }));
                // insert the node into the previous hash map
                if previous.contains_key(&neighbor) {
                    let mut path = previous.get(&neighbor).unwrap().clone();
                    path.push(node.clone());
                    previous.insert(neighbor.clone(), path);
                } else {
                    let mut path = Vec::new();
                    path.push(node.clone());
                    previous.insert(neighbor.clone(), path);
                }
            }
        }
    }
    if found_path {
        return Some(previous);
    }
    // return None if no path is found
    None
}

// cursor positioning for the terminal
macro_rules! POS {
    () => { "\x1B[{};{}H" };
}

macro_rules! UP {
    () => { "\x1B[{}A" };
}

macro_rules! DOWN {
    () => { "\x1B[{}B" };
}

macro_rules! RIGHT {
    () => { "\x1B[{}C" };
}

macro_rules! LEFT {
    () => { "\x1B[{}D" };
}

macro_rules! CLEAR {
    () => { "\x1B[2J" };
}

macro_rules! ERASE_TO_EOL {
    () => { "\x1B[K" };
}

pub fn position(x: u32, y: u32) {
    print!("{}", format!(POS!(), y+1, x+1));
}

pub fn up(n: u32) {
    print!("{}", format!(UP!(), n));
}

pub fn down(n: u32) {
    print!("{}", format!(DOWN!(), n));
}

pub fn right(n: u32) {
    print!("{}", format!(RIGHT!(), n));
}

pub fn left(n: u32) {
    print!("{}", format!(LEFT!(), n));
}

pub fn clear() {
    print!("{}", CLEAR!());
}

pub fn erase_to_eol() {
    print!("{}", ERASE_TO_EOL!());
}

// define a generic interval type

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval<T> {
    pub start: T,
    pub end: T,
}

impl<T> Interval<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}

// intersection
pub fn intersect<T: PartialOrd + Copy + Ord>(a: Interval<T>, b: Interval<T>) -> Option<Interval<T>> {
    if a.start < b.end && b.start < a.end {
        Some(Interval::new(std::cmp::max(a.start, b.start), std::cmp::min(a.end, b.end)))
    } else {
        None
    }
}

// union
pub fn union<T: PartialOrd + Copy + Ord>(a: Interval<T>, b: Interval<T>) -> Interval<T> {
    Interval::new(std::cmp::min(a.start, b.start), std::cmp::max(a.end, b.end))
}

// difference
pub fn difference<T: PartialOrd + Copy>(a: Interval<T>, b: Interval<T>) -> Vec<Interval<T>> {
    if a.start >= b.end || b.start >= a.end {
        vec![a]
    } else if a.start < b.start && a.end > b.end {
        vec![Interval::new(a.start, b.start), Interval::new(b.end, a.end)]
    } else if a.start < b.start {
        vec![Interval::new(a.start, b.start)]
    } else {
        vec![Interval::new(b.end, a.end)]
    }
}

// contains
pub fn contains<T: PartialOrd + Copy>(a: Interval<T>, b: Interval<T>) -> bool {
    a.start <= b.start && a.end >= b.end
}

// equals
pub fn equals<T: PartialEq + Copy>(a: Interval<T>, b: Interval<T>) -> bool {
    a.start == b.start && a.end == b.end
}

// to_string
pub fn to_string<T: std::string::ToString>(interval: Interval<T>) -> String {
    format!("[{}, {}]", interval.start.to_string(), interval.end.to_string())
}

// less
pub fn less<T: PartialOrd + Copy>(a: Interval<T>, b: Interval<T>) -> bool {
    a.start < b.start || (a.start == b.start && a.end < b.end)
}

// We can run the A* algorithm on a graph of nodes, if:
// 1. we can get a name for the node to hash by
// 2. get all neighbors of a node
// 3. get the cost (or weight) of the edges
// 4. have a heuristic function to guess at the cost to a target node
pub fn a_star<Node, NeighborFn, EdgeWeightFn, HeuristicFn>(start: Node, goal: Node, get_neighbors: NeighborFn, weight_fn: EdgeWeightFn, get_heuristic: HeuristicFn) -> Option<Vec<Node>>
where
    Node: std::cmp::Eq + std::hash::Hash + std::clone::Clone + std::fmt::Debug,
    NeighborFn: Fn(&Node) -> Vec<Node>,
    EdgeWeightFn: Fn(&Node, &Node) -> u64,
    HeuristicFn: Fn(&Node, &Node) -> u64,
{
    // create a priority queue
    let mut queue = std::collections::BinaryHeap::new();
    // create a hash map to store the distance to each node
    let mut distance = std::collections::HashMap::new();
    // create a hash map to store the previous node in the path
    let mut previous: HashMap<Node,Node> = std::collections::HashMap::new();
    // create a hash set to store the nodes that have been visited
    let mut visited = std::collections::HashSet::new();
    // insert the start node into the queue with a distance of zero
    queue.push(Reverse(PriorityNode { priority: 0, node: start.clone() }));
    // if debugging, print the queue
    println!("Queue: {:?}", queue);
    // set the distance to the start node to zero
    distance.insert(start.clone(), 0);
    // while the queue is not empty
    while let Some(Reverse(PriorityNode { priority: dist, node })) = queue.pop() {
        // if the node is the goal node
        if node == goal {
            // create a vector to store the path
            let mut path = Vec::new();
            // set the current node to the goal node
            let mut current = goal.clone();
            // while the current node is not the start node
            while current != start {
                // insert the current node into the path vector
                path.push(current.clone());
                // set the current node to the previous node
                current = previous[&current].clone();
            }
            // insert the start node into the path vector
            path.push(start.clone());
            // reverse the path vector
            path.reverse();
            // return the path vector
            return Some(path);
        }
        // if the node has been visited
        if visited.contains(&node) {
            // skip the rest of the loop
            continue;
        }
        // insert the node into the visited set
        visited.insert(node.clone());
        // for each neighbor of the node
        for neighbor
        in get_neighbors(&node) {
            // calculate the new distance to the neighbor
            let new_distance = dist + weight_fn(&node, &neighbor);
            // if the neighbor has not been visited or the new distance is less than the current distance
            if !visited.contains(&neighbor) || new_distance < *distance.get(&neighbor).unwrap_or(&u64::MAX) {
                // insert the new distance into the distance hash map
                distance.insert(neighbor.clone(), new_distance);
                // insert the neighbor into the queue with the new distance
                queue.push(Reverse(PriorityNode { priority: new_distance + get_heuristic(&neighbor, &goal), node: neighbor.clone() }));
                // insert the node into the previous hash map
                previous.insert(neighbor.clone(), node.clone());
            }
        }
        // println!("Queue: {:?}", queue);
        // println!("Distances: {:?}", distance);
        // println!();
    }
    // return None if no path is found
    None
}

// define generic points and vectors in 2D space
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: &Point2D<T>) -> Point2D<T>
    where
        T: std::ops::Add<Output = T> + Copy,
    {
        Point2D::new(self.x + other.x, self.y + other.y)
    }

    pub fn subtract(&self, other: &Point2D<T>) -> Point2D<T>
    where
        T: std::ops::Sub<Output = T> + Copy,
    {
        Point2D::new(self.x - other.x, self.y - other.y)
    }

    pub fn scale(&self, scalar: T) -> Point2D<T>
    where
        T: std::ops::Mul<Output = T> + Copy,
    {
        Point2D::new(self.x * scalar, self.y * scalar)
    }

    pub fn manhattan_distance(&self, other: &Point2D<T>) -> T
    where
        T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy + Signed,
    {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn euclidean_distance(&self, other: &Point2D<T>) -> f64
    where
        T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
        f64: std::convert::From<T>,
    {
        let dx = f64::from(self.x - other.x);
        let dy = f64::from(self.y - other.y);
        (dx * dx + dy * dy).sqrt()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path() {
        #[derive(Debug, Clone, Eq, PartialEq, Hash)]
        struct Node {
            id: u32,
        }

        let start = Node { id: 1 };
        let goal = Node { id: 4 };

        let get_neighbors = |node: &Node| -> Vec<Node> {
            match node.id {
                1 => vec![Node { id: 2 }, Node { id: 3 }],
                2 => vec![Node { id: 4 }],
                3 => vec![Node { id: 4 }],
                _ => vec![],
            }
        };

        let get_distance = |_node1: &Node, _node2: &Node| -> u64 {
            1
        };

        let get_heuristic = |_node1: &Node, _node2: &Node| -> u64 {
            0
        };

        let path = shortest_path(start.clone(), goal.clone(), get_neighbors, get_distance, get_heuristic);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path, vec![start, Node { id: 2 }, goal]);
    }

    // test the interval functions
    #[test]
    fn test_interval_intersect() {
        let a = Interval::new(1, 5);
        let b = Interval::new(3, 7);
        let c = Interval::new(6, 8);

        assert_eq!(intersect(a, b), Some(Interval::new(3, 5)));
        assert_eq!(intersect(a, c), None);
    }

    #[test]
    fn test_interval_union() {
        let a = Interval::new(1, 5);
        let b = Interval::new(3, 7);

        assert_eq!(union(a, b), Interval::new(1, 7));
    }

    #[test]
    fn test_interval_difference() {
        let a = Interval::new(1, 5);
        let b = Interval::new(3, 7);
        let c = Interval::new(0, 2);

        assert_eq!(difference(a, b), vec![Interval::new(1, 3)]);
        assert_eq!(difference(a, c), vec![Interval::new(2, 5)]);
    }

    #[test]
    fn test_interval_contains() {
        let a = Interval::new(1, 5);
        let b = Interval::new(2, 4);
        let c = Interval::new(0, 6);

        assert!(contains(a, b));
        assert!(!contains(a, c));
    }

    #[test]
    fn test_interval_equals() {
        let a = Interval::new(1, 5);
        let b = Interval::new(1, 5);
        let c = Interval::new(2, 6);

        assert!(equals(a, b));
        assert!(!equals(a, c));
    }

    #[test]
    fn test_interval_to_string() {
        let a = Interval::new(1, 5);
        assert_eq!(to_string(a), "[1, 5]");
    }

    #[test]
    fn test_interval_less() {
        let a = Interval::new(1, 5);
        let b = Interval::new(2, 6);
        let c = Interval::new(1, 4);

        assert!(less(a, b));
        assert!(!less(a, c));
    }
    
    // test the A* algorithm
    #[test]
    fn test_a_star() {
        #[derive(Debug, Clone, Eq, PartialEq, Hash)]
        struct Node {
            id: u32,
        }

        let start = Node { id: 1 };
        let goal = Node { id: 4 };

        let get_neighbors = |node: &Node| -> Vec<Node> {
            match node.id {
                1 => vec![Node { id: 2 }, Node { id: 3 }],
                2 => vec![Node { id: 4 }],
                3 => vec![Node { id: 4 }],
                _ => vec![],
            }
        };

        let weight_fn = |_node1: &Node, _node2: &Node| -> u64 {
            1
        };

        let heuristic_fn = |_node1: &Node, _node2: &Node| -> u64 {
            0
        };

        let path = a_star(start.clone(), goal.clone(), get_neighbors, weight_fn, heuristic_fn);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path, vec![start, Node { id: 2 }, goal]);
    }

    // add tests for the Point2D struct
    #[test]
    fn test_point2d_add() {
        let a = Point2D::new(1, 2);
        let b = Point2D::new(3, 4);
        let c = a.add(&b);
        assert_eq!(c, Point2D::new(4, 6));
    }

    #[test]
    fn test_point2d_subtract() {
        let a = Point2D::new(1, 2);
        let b = Point2D::new(3, 4);
        let c = a.subtract(&b);
        assert_eq!(c, Point2D::new(-2, -2));
    }

    #[test]
    fn test_point2d_manhattan_distance() {
        let a = Point2D::new(1, 2);
        let b = Point2D::new(3, 4);
        let c = a.manhattan_distance(&b);
        assert_eq!(c, 4);
    }

    #[test]
    fn test_point2d_euclidean_distance() {
        let a = Point2D::new(1, 2);
        let b = Point2D::new(3, 4);
        let c = a.euclidean_distance(&b);
        assert_eq!(c, 2.8284271247461903);
    }
}
