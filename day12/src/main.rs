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

// turn off warning for unused variables
#[allow(unused_variables)]
pub fn part1(contents: &String) -> String {
    let garden = make_garden(contents);
    let mut cost = 0;
    let mut visited = Vec::new();
    for y in 0..garden.len() {
        for x in 0..garden[y].len() {
            cost += region_cost(&garden, x, y, &mut visited);
        }
    }
    cost.to_string()
}

fn region_cost(garden: &Garden, x: usize, y: usize, visited: &mut Vec<(usize, usize)>) -> i32 {
    if visited.contains(&(x, y)) {
        return 0;
    }
    let mut perimeter = 0;
    let mut stack = Vec::new();
    let mut count = 0;
    stack.push((x, y));
    while stack.len() > 0 {
        let (x, y) = stack.pop().unwrap();
        if visited.contains(&(x, y)) {
            continue;
        }
        count += 1;
        visited.push((x, y));
        perimeter += 4-garden[y][x].neighbors;
        if x > 0 {
            if garden[y][x].plant == garden[y][x-1].plant {
                stack.push((x - 1, y));
            }
        }
        if x < garden[0].len() - 1 {
            if garden[y][x].plant == garden[y][x+1].plant {
                stack.push((x + 1, y));
            }
        }
        if y > 0 {
            if garden[y][x].plant == garden[y-1][x].plant {
                stack.push((x, y - 1));
            }
        }
        if y < garden.len() - 1 {
            if garden[y][x].plant == garden[y+1][x].plant {
                stack.push((x, y + 1));
            }
        }
    }
    println!("Region: {}, {}", garden[y][x].plant, count * perimeter);
    count * perimeter
}

struct Plot {
    plant: char,
    neighbors: i32
}

type Garden = Vec<Vec<Plot>>;

fn make_garden(contents: &String) -> Garden {
    let mut grid = Vec::new();
    for line in contents.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Plot { plant: c, neighbors: 0 });
        }
        grid.push(row);
    }
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            // count neighbors that have the same plant
            let neighbors = check_neighbor(&grid, x, y, -1, 0) +
                            check_neighbor(&grid, x, y, 1, 0) +
                            check_neighbor(&grid, x, y, 0, -1) +
                            check_neighbor(&grid, x, y, 0, 1);
            grid[y][x].neighbors = neighbors;
        }
    }
    grid
}

fn check_neighbor(grid: &Garden, x: usize, y: usize, dx: i32, dy: i32) -> i32 {
    let nx = x as i32 + dx;
    let ny = y as i32 + dy;
    if nx < 0 || ny < 0 || x >= grid[0].len() || nx >= grid[0].len() as i32 || ny >= grid.len() as i32 {
        return 0;
    }
    if grid[y as usize][x as usize].plant == grid[ny as usize][nx as usize].plant {
        return 1;
    }
    0
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
    fn test2_1() {
        // get the contents of the file "files/test2"
        let contents = advent::read_file_to_string("files/test2");
        // call part1 with the contents of the file
        let result = part1(&contents);
        // get the contents of the file "files/test_answer_1"
        let answer = advent::read_file_to_string("files/test2_answer_1");
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
