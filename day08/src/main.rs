use std::time::Instant;
use std::collections::{HashMap, HashSet};

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
    let (grid, node_map) = parse_input(contents);
    let antinodes = get_antinodes(&grid, &node_map);
    print_antinodes_on_grid(&grid, &antinodes);
    // get the count of antinodes
    let result = antinodes.len();
    result.to_string()
}

type Point = (i32, i32);
type NodeMap = HashMap<char, Vec<Point>>;

fn print_antinodes_on_grid(grid: &Vec<Vec<char>>, antinodes: &HashSet<Point>) {
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let point = (x as i32, y as i32);
            if antinodes.contains(&point) {
                print!("#");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

fn get_antinodes(grid: &Vec<Vec<char>>, node_map: &NodeMap) -> HashSet<Point> {
    let width = grid[0].len();
    let height = grid.len();
    let mut antinodes = HashSet::new();
    for (_node, points) in node_map.iter() {
        for point1 in points {
            for point2 in points {
                if point1 == point2 {
                    continue;
                }
                // get the direction from point1 to point2
                let direction = (point2.0 - point1.0, point2.1 - point1.1);
                // go further in that direction past point2
                let antinode1 = (point2.0 + direction.0, point2.1 + direction.1);
                // go the negative direction past point1
                let antinode2 = (point1.0 - direction.0, point1.1 - direction.1);
                // check if the antinodes are in bounds
                if antinode1.0 >= 0 && antinode1.0 < width as i32 && antinode1.1 >= 0 && antinode1.1 < height as i32 {
                    antinodes.insert(antinode1);
                }
                if antinode2.0 >= 0 && antinode2.0 < width as i32 && antinode2.1 >= 0 && antinode2.1 < height as i32 {
                    antinodes.insert(antinode2);
                }
            }
        }
    }
    antinodes
}

fn get_antinodes_with_harmonics(grid: &Vec<Vec<char>>, node_map: &NodeMap) -> HashSet<Point> {
    let width = grid[0].len();
    let height = grid.len();
    let mut antinodes = HashSet::new();
    for (_node, points) in node_map.iter() {
        if points.len() < 2 {
            continue;
        }
        for point1 in points {
            for point2 in points {
                if point1 == point2 {
                    continue;
                }
                antinodes.insert(*point1);
                antinodes.insert(*point2);
                // get the direction from point1 to point2
                let direction = (point2.0 - point1.0, point2.1 - point1.1);
                // go further in that direction past point2
                let mut current_point = *point2;
                loop {
                    let antinode = (current_point.0 + direction.0, current_point.1 + direction.1);
                    // check if the antinode is in bounds
                    if antinode.0 < 0 || antinode.0 >= width as i32 || antinode.1 < 0 || antinode.1 >= height as i32 {
                        break;
                    }
                    current_point = antinode;
                    antinodes.insert(antinode);
                }
                current_point = *point1;
                loop {
                    let antinode = (current_point.0 - direction.0, current_point.1 - direction.1);
                    // check if the antinode is in bounds
                    if antinode.0 < 0 || antinode.0 >= width as i32 || antinode.1 < 0 || antinode.1 >= height as i32 {
                        break;
                    }
                    current_point = antinode;
                    antinodes.insert(antinode);
                }
            }
        }
    }
    antinodes
}

fn parse_input(contents: &String) -> (Vec<Vec<char>>, NodeMap) {
    let mut grid = Vec::new();
    let mut node_map: NodeMap = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                _ => {
                    // add the current point to the node map
                    node_map.entry(c).or_insert(Vec::new()).push((x as i32, y as i32));
                }
            }
        }
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }
    (grid, node_map)
}

#[allow(unused_variables)]
pub fn part2(contents: &String) -> String {
    let (grid, node_map) = parse_input(contents);
    let antinodes = get_antinodes_with_harmonics(&grid, &node_map);
    print_antinodes_on_grid(&grid, &antinodes);
    // get the count of antinodes
    let result = antinodes.len();
    result.to_string()
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
