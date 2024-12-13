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
    let garden = make_garden(contents);
    let mut cost = 0;
    let mut visited = HashSet::new();
    for y in 0..garden.len() {
        for x in 0..garden[y].len() {
            cost += region_cost(&garden, x, y, &mut visited);
        }
    }
    cost.to_string()
}

fn region_cost(garden: &Garden, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) -> i32 {
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
        visited.insert((x, y));
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
    // println!("Region: {}, {}", garden[y][x].plant, count * perimeter);
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
    let garden = make_garden(contents);
    let mut cost = 0;
    let mut visited = HashSet::new();
    for y in 0..garden.len() {
        for x in 0..garden[y].len() {
            cost += discount_cost(&garden, x, y, &mut visited);
        }
    }
    cost.to_string()
}

fn discount_cost(garden: &Garden, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) -> i32 {
    if visited.contains(&(x, y)) {
        return 0;
    }
    // let mut count = 0;
    // let mut sides = 0;
    let mut one_block = HashSet::new();
    let mut stack = Vec::new();
    let (mut minx, mut miny, mut maxx, mut maxy) = (x, y, x, y);

    one_block.insert((x, y));
    stack.push((x, y));
    while stack.len() > 0 {
        let (x, y) = stack.pop().unwrap();
        if visited.contains(&(x, y)) {
            continue;
        }
        // count += 1;
        visited.insert((x, y));
        one_block.insert((x, y));
        minx = minx.min(x);
        miny = miny.min(y);
        maxx = maxx.max(x);
        maxy = maxy.max(y);
        if x > 0 {
            if garden[y][x].plant == garden[y][x-1].plant && !visited.contains(&(x-1, y)) {
                stack.push((x - 1, y));
            }
        }
        if x < garden[0].len() - 1 {
            if garden[y][x].plant == garden[y][x+1].plant && !visited.contains(&(x+1, y)) {
                stack.push((x + 1, y));
            }
        }
        if y > 0 {
            if garden[y][x].plant == garden[y-1][x].plant && !visited.contains(&(x, y-1)) {
                stack.push((x, y - 1));
            }
        }
        if y < garden.len() - 1 {
            if garden[y][x].plant == garden[y+1][x].plant && !visited.contains(&(x, y+1)) {
                stack.push((x, y + 1));
            }
        }
    }

    let (count, sides) = find_sides(&garden, minx, miny, maxx, maxy, &one_block);
    
    count * sides
}

fn find_sides(garden: &Garden, minx: usize, miny: usize, maxx: usize, maxy: usize, one_block: &HashSet<(usize, usize)>) -> (i32, i32) {
    let mut sides = 0;
    let count = one_block.len() as i32;

    // top sides
    for y in miny..=maxy {
        let mut x = minx;
        while x <= maxx {
            if one_block.contains(&(x, y)) && has_border(&garden, x as i32, y as i32, 0, -1) {
                sides += 1;
                x += 1;
                while x <= maxx && one_block.contains(&(x, y)) && has_border(&garden, x as i32, y as i32, 0, -1) {
                    x += 1;
                }
            } else {
                x += 1;
            }
        }
    }

    // bottom sides
    for y in miny..=maxy {
        let mut x = minx;
        while x <= maxx {
            if one_block.contains(&(x, y)) && has_border(&garden, x as i32, y as i32, 0, 1) {
                sides += 1;
                x += 1;
                while x <= maxx && one_block.contains(&(x, y)) && has_border(&garden, x as i32, y as i32, 0, 1) {
                    x += 1;
                }
            } else {
                x += 1;
            }
        }
    }

    // left sides
    for x in minx..=maxx {
        let mut y = miny;
        while y<= maxy {
            if one_block.contains(&(x, y)) && has_border(&garden, x as i32, y as i32, -1, 0) {
                sides += 1;
                y += 1;
                while y <= maxy && one_block.contains(&(x, y)) && has_border(&garden, x as i32, y as i32, -1, 0) {
                    y += 1;
                }
            } else {
                y += 1;
            }
        }
    }

    // right sides
    for x in minx..=maxx {
        let mut y = miny;
        while y <= maxy {
            if one_block.contains(&(x, y)) && has_border(&garden, x as i32, y as i32, 1, 0) {
                sides += 1;
                y += 1;
                while y <= maxy && one_block.contains(&(x, y)) && has_border(&garden, x as i32, y as i32, 1, 0) {
                    y += 1;
                }
            } else {
                y += 1;
            }
        }
    }

    (count, sides)
}

fn has_border(grid: &Garden, x: i32, y: i32, dx: i32, dy: i32) -> bool {
    let nx = x + dx;
    let ny = y + dy;
    if nx < 0 || ny < 0 || nx >= grid[0].len() as i32 || ny >= grid.len() as i32 {
        return true;
    }
    let p1 = grid[ny as usize][nx as usize].plant;
    let p2 = grid[y as usize][x as usize].plant;
    p1 != p2
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
