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
    let sections = contents.split("\n\n").collect::<Vec<&str>>();
    let grid_input = sections[0];
    let move_input = sections[1];
    let mut grid = make_grid(grid_input,1);
    let moves = move_input.chars().collect::<Vec<char>>();
    simulate(&mut grid, moves);
    score_grid(&grid).to_string()
}

type Point = (i32, i32);

struct Grid {
    grid: Vec<Vec<char>>,
    width: i32,
    height: i32,
    robot: Point,
}

fn move_object(grid: &mut Grid, p: Point, m: char) -> bool {
    if grid.grid[p.1 as usize][p.0 as usize] == '#' {
        return false;
    }
    if grid.grid[p.1 as usize][p.0 as usize] == '.' {
        return true;
    }
    let next_p: Point;
    match m {
        '^' => {
            next_p = (p.0, p.1 - 1);
            match grid.grid[next_p.1 as usize][next_p.0 as usize] {
                '[' => {
                    // we have to check this plus the next to the right
                    let right: Point = (next_p.0 + 1, next_p.1);
                    if move_object(grid, next_p,m) && move_object(grid, right,m) {
                        if grid.grid[p.1 as usize][p.0 as usize] == '@' {
                            grid.robot = next_p;
                        }
                        grid.grid[next_p.1 as usize][next_p.0 as usize] = grid.grid[p.1 as usize][p.0 as usize];
                        grid.grid[p.1 as usize][p.0 as usize] = '.';
                        return true;
                    }
                }
                ']' => {
                    // we have to check this plus the next to the left
                    let left: Point = (next_p.0 - 1, next_p.1);
                    if move_object(grid, next_p,m) && move_object(grid, left,m) {
                        if grid.grid[p.1 as usize][p.0 as usize] == '@' {
                            grid.robot = next_p;
                        }
                        grid.grid[next_p.1 as usize][next_p.0 as usize] = grid.grid[p.1 as usize][p.0 as usize];
                        grid.grid[p.1 as usize][p.0 as usize] = '.';
                        return true;
                    }
                }
                _ => {
                    if move_object(grid, next_p, m) {
                        if grid.grid[p.1 as usize][p.0 as usize] == '@' {
                            grid.robot = next_p;
                        }
                        grid.grid[next_p.1 as usize][next_p.0 as usize] = grid.grid[p.1 as usize][p.0 as usize];
                        grid.grid[p.1 as usize][p.0 as usize] = '.';
                        return true;
                    }
                }
            }
        }
        '>' => {
            next_p = (p.0 + 1, p.1);
            if move_object(grid, next_p, m) {
                if grid.grid[p.1 as usize][p.0 as usize] == '@' {
                    grid.robot = next_p;
                }
                grid.grid[next_p.1 as usize][next_p.0 as usize] = grid.grid[p.1 as usize][p.0 as usize];
                grid.grid[p.1 as usize][p.0 as usize] = '.';
                return true;
            }
        }
        'v' => {
            next_p = (p.0, p.1 + 1);
            match grid.grid[next_p.1 as usize][next_p.0 as usize] {
                '[' => {
                    // we have to check this plus the next to the right
                    let right: Point = (next_p.0 + 1, next_p.1);
                    if move_object(grid, next_p,m) && move_object(grid, right,m) {
                        if grid.grid[p.1 as usize][p.0 as usize] == '@' {
                            grid.robot = next_p;
                        }
                        grid.grid[next_p.1 as usize][next_p.0 as usize] = grid.grid[p.1 as usize][p.0 as usize];
                        grid.grid[p.1 as usize][p.0 as usize] = '.';
                        return true;
                    }
                }
                ']' => {
                    // we have to check this plus the next to the left
                    let left: Point = (next_p.0 - 1, next_p.1);
                    if move_object(grid, next_p,m) && move_object(grid, left,m) {
                        if grid.grid[p.1 as usize][p.0 as usize] == '@' {
                            grid.robot = next_p;
                        }
                        grid.grid[next_p.1 as usize][next_p.0 as usize] = grid.grid[p.1 as usize][p.0 as usize];
                        grid.grid[p.1 as usize][p.0 as usize] = '.';
                        return true;
                    }
                }
                _ => {
                    if move_object(grid, next_p, m) {
                        if grid.grid[p.1 as usize][p.0 as usize] == '@' {
                            grid.robot = next_p;
                        }
                        grid.grid[next_p.1 as usize][next_p.0 as usize] = grid.grid[p.1 as usize][p.0 as usize];
                        grid.grid[p.1 as usize][p.0 as usize] = '.';
                        return true;
                    }
                }
            }
        }
        '<' => {
            next_p = (p.0 - 1, p.1);
            if move_object(grid, next_p, m) {
                if grid.grid[p.1 as usize][p.0 as usize] == '@' {
                    grid.robot = next_p;
                }
                grid.grid[next_p.1 as usize][next_p.0 as usize] = grid.grid[p.1 as usize][p.0 as usize];
                grid.grid[p.1 as usize][p.0 as usize] = '.';
                return true;
            }
        }
        _ => {
            return false;
        }
    }
    false
}

fn can_move_object(grid: &Grid, p: Point, m: char) -> bool {
    if grid.grid[p.1 as usize][p.0 as usize] == '#' {
        return false;
    }
    if grid.grid[p.1 as usize][p.0 as usize] == '.' {
        return true;
    }
    let next_p: Point;
    match m {
        '^' => {
            next_p = (p.0, p.1 - 1);
            match grid.grid[next_p.1 as usize][next_p.0 as usize] {
                '[' => {
                    // we have to check this plus the next to the right
                    let right: Point = (next_p.0 + 1, next_p.1);
                    if can_move_object(grid, next_p,m) && can_move_object(grid, right,m) {
                        return true;
                    }
                }
                ']' => {
                    // we have to check this plus the next to the left
                    let left: Point = (next_p.0 - 1, next_p.1);
                    if can_move_object(grid, next_p,m) && can_move_object(grid, left,m) {
                        return true;
                    }
                }
                _ => {
                    if can_move_object(grid, next_p, m) {
                        return true;
                    }
                }
            }
        }
        '>' => {
            next_p = (p.0 + 1, p.1);
            if can_move_object(grid, next_p, m) {
                return true;
            }
        }
        'v' => {
            next_p = (p.0, p.1 + 1);
            match grid.grid[next_p.1 as usize][next_p.0 as usize] {
                '[' => {
                    // we have to check this plus the next to the right
                    let right: Point = (next_p.0 + 1, next_p.1);
                    if can_move_object(grid, next_p,m) && can_move_object(grid, right,m) {
                        return true;
                    }
                }
                ']' => {
                    // we have to check this plus the next to the left
                    let left: Point = (next_p.0 - 1, next_p.1);
                    if can_move_object(grid, next_p,m) && can_move_object(grid, left,m) {
                        return true;
                    }
                }
                _ => {
                    if can_move_object(grid, next_p, m) {
                        return true;
                    }
                }
            }
        }
        '<' => {
            next_p = (p.0 - 1, p.1);
            if can_move_object(grid, next_p, m) {
                return true;
            }
        }
        _ => {
            return false;
        }
    }
    false
}

fn simulate(grid: &mut Grid, moves: Vec<char>) {
    for m in moves {
        // print_grid(grid);
        if can_move_object(grid, grid.robot, m) {
            move_object(grid, grid.robot, m);
        }
    }
    print_grid(grid);
}

fn print_grid(grid: &Grid) {
    for row in grid.grid.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn score_grid(grid: &Grid) -> i32 {
    let mut sum = 0;
    for (y, row) in grid.grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'O' || *c == '[' {
                sum += (y as i32 * 100) + (x as i32);
            }
        }
    }
    sum
}

fn make_grid(grid_input: &str,factor: i32) -> Grid {
    let mut grid = Grid {
        grid: Vec::new(),
        width: 0,
        height: 0,
        robot: (0, 0),
    };
    let lines = grid_input.split("\n").collect::<Vec<&str>>();
    grid.height = lines.len() as i32;
    for (y,line) in lines.iter().enumerate() {
        let mut row = Vec::new();
        let chars = line.chars().collect::<Vec<char>>();
        grid.width = (chars.len() as i32) * factor;
        for (x, c) in chars.iter().enumerate() {
            if *c == '@' {
                grid.robot = (factor*x as i32, y as i32);  
                row.push(*c);
                if factor > 1 {
                    row.push('.');
                }
            } else if *c == 'O' {
                if factor == 1 {
                    row.push('O');
                } else {
                    row.push('[');
                    row.push(']');
                }
            } else {
                for _ in 0..factor {
                    row.push(*c);
                }
            }
        }
        grid.grid.push(row);
    }
    grid
}

#[allow(unused_variables)]
pub fn part2(contents: &String) -> String {
    let sections = contents.split("\n\n").collect::<Vec<&str>>();
    let grid_input = sections[0];
    let move_input = sections[1];
    let mut grid = make_grid(grid_input,2);
    let moves = move_input.chars().collect::<Vec<char>>();
    simulate(&mut grid, moves);
    score_grid(&grid).to_string()
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
