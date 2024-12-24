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
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let mut nodes: HashMap<&str,i32> = HashMap::new();
    let mut edges: HashMap<&str,Vec<&str>> = HashMap::new();
    for line in lines {
        let parts = line.split("-").collect::<Vec<&str>>();
        let from = parts[0];
        let to = parts[1];
        if !nodes.contains_key(from) {
            nodes.insert(from, 0);
        }
        // check if the list of edges contains the from node
        if !edges.contains_key(from) {
            // create a new vector of strings and insert it into the edges hashmap
            edges.insert(from, Vec::new());
        }
        // add the to node to the vector of strings
        edges.get_mut(from).unwrap().push(to);
        // check if the list of edges contains the to node
        if !edges.contains_key(to) {
            // create a new vector of strings and insert it into the edges hashmap
            edges.insert(to, Vec::new());
        }
        // add the from node to the vector of strings
        edges.get_mut(to).unwrap().push(from);
    }
    count_t_triangles(&edges).to_string()
}

fn count_t_triangles(edges: &HashMap<&str,Vec<&str>>) -> i32 {
    let mut count = 0;
    let mut visited: HashSet<(&str,&str,&str)> = HashSet::new();
    for (from, to) in edges {
        if from.starts_with("t") {
            for node in to {
                for node2 in edges.get(node).unwrap() {
                    if to.contains(node2) {
                        let mut v = vec![from, node, node2];
                        v.sort();
                        if !visited.contains(&(v[0],v[1],v[2])) {
                            visited.insert((v[0],v[1],v[2]));
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    count
}

#[allow(unused_variables)]
pub fn part2(contents: &String) -> String {
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let mut nodes: HashMap<&str,i32> = HashMap::new();
    let mut edges: HashMap<&str,Vec<&str>> = HashMap::new();
    for line in lines {
        let parts = line.split("-").collect::<Vec<&str>>();
        let from = parts[0];
        let to = parts[1];
        if !nodes.contains_key(from) {
            nodes.insert(from, 0);
        }
        // check if the list of edges contains the from node
        if !edges.contains_key(from) {
            // create a new vector of strings and insert it into the edges hashmap
            edges.insert(from, Vec::new());
        }
        // add the to node to the vector of strings
        edges.get_mut(from).unwrap().push(to);
        // check if the list of edges contains the to node
        if !edges.contains_key(to) {
            // create a new vector of strings and insert it into the edges hashmap
            edges.insert(to, Vec::new());
        }
        // add the from node to the vector of strings
        edges.get_mut(to).unwrap().push(from);
    }
    let mut all_visited: HashSet<&str> = HashSet::new();
    let mut biggest_network = (HashSet::new(), 0);
    for (node, _) in &nodes {
        if all_visited.contains(node) {
            continue;
        }
        let visited = network(node, &edges);
        if visited.len() > biggest_network.1 {
            biggest_network = (visited.clone(), visited.len());
        }
        for v in visited {
            all_visited.insert(v);
        }
    }
    // get a Vector of all the nodes from the biggest network, sort them, and join them into a string separated by commas
    let mut v = biggest_network.0.into_iter().collect::<Vec<&str>>();
    v.sort();
    let result = v.iter().enumerate().map(|(i, x)| {
        if i == 0 {
            x.to_string()
        } else {
            format!(",{}", x)
        }
    }).collect::<String>(); 

    result.to_string()
}

fn network<'a>(start: &'a str, edges: &'a HashMap<&'a str, Vec<&'a str>>) -> HashSet<&'a str> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut clique: HashSet<&str> = HashSet::new();
    let mut stack: Vec<&str> = Vec::new();
    stack.push(start);
    clique.insert(start);
    while stack.len() > 0 {
        let node = stack.pop().unwrap();
        if !visited.contains(node) {
            // check if the new node is a neighbor to every node in the clique
            let mut in_clique = true;
            let links = edges.get(node).unwrap();
            for n in &clique {
                if !links.contains(&n) {
                    in_clique = false;
                    break;
                }
            }
            if in_clique {
                clique.insert(node);
            }
            visited.insert(node);
            for n in edges.get(node).unwrap() {
                stack.push(n);
            }
        }
    }
    println!("{:?}", clique);
    clique
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
