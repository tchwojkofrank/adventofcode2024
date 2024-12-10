use std::{collections::HashSet, time::Instant};

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
    let grid = parse_input(contents);
    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 0 {
                let mut peaks = HashSet::new();
                let start = Point{x: x as i32, y: y as i32};
                _ = good_trails_from_here(&grid, &start, -1, &mut peaks);
                count += peaks.len();
            }
        }
    }
    count.to_string()
}
#[derive(Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn good_trails_from_here(grid: &Vec<Vec<i32>>, pos: &Point, prev: i32, peaks: &mut HashSet<Point>) -> i32 {
    if pos.y < 0 || pos.y >= grid.len() as i32 || pos.x < 0 || pos.x >= grid[0].len() as i32 {
        return 0;
    }

    if grid[pos.y as usize][pos.x as usize] != prev+1 {
        return 0;
    }

    if grid[pos.y as usize][pos.x as usize] == 9 {
        peaks.insert(Point{x: pos.x, y: pos.y});
        return 1;
    }

    let mut count = 0;
    // check all four directions
    let north = Point{x: pos.x, y: pos.y-1};
    let east: Point = Point{x: pos.x+1, y: pos.y};
    let south: Point = Point{x: pos.x, y: pos.y+1};
    let west: Point = Point{x: pos.x-1, y: pos.y};
    count += good_trails_from_here(grid, &north, grid[pos.y as usize][pos.x as usize], peaks);
    count += good_trails_from_here(grid, &east, grid[pos.y as usize][pos.x as usize], peaks);
    count += good_trails_from_here(grid, &south, grid[pos.y as usize][pos.x as usize], peaks);
    count += good_trails_from_here(grid, &west, grid[pos.y as usize][pos.x as usize], peaks);

    count
}

fn parse_input(contents: &String) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for line in contents.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            // push the value of the character into the row
            row.push(c as i32 - '0' as i32);
        }
        result.push(row);
    }
    result
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
