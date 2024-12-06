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
    let result1 = part1(&contents);
    println!("Part 1: {}", result1);
    let result2 = part2(&contents);
    println!("Part 2: {}", result2);
}

// turn off warning for unused variables
#[allow(unused_variables)]
pub fn part1(contents: &String) -> String {
    let (mut grid, guard_position, guard_direction) = parse_input(contents);
    let result = track_guard_path(&mut grid, &guard_position, &guard_direction);
    result.to_string()
}

const N: Direction = (0,-1);
const E: Direction = (1,0);
const S: Direction = (0,1);
const W: Direction = (-1,0);

// define a type to hold a 2d grid of data
type Grid = Vec<Vec<char>>;
type Position = (i32, i32);
type Direction = (i32, i32);

fn parse_input(contents: &String) -> (Grid, Position, Direction) {
    let mut grid = Grid::new();
    let mut guard_position = (0, 0);
    let mut guard_direction = N;
    for (y, line) in contents.lines().enumerate() {
        let row: Vec<char> = line.chars().enumerate().map(|(x, c)| {
            match c {
                '^' => {
                    guard_position = (x as i32, y as i32);
                    guard_direction = N;
                    '.' // replace the guard with an empty space
                },
                'v' => {
                    guard_position = (x as i32, y as i32);
                    guard_direction = S;
                    '.' // replace the guard with an empty space
                },
                '<' => {
                    guard_position = (x as i32, y as i32);
                    guard_direction = W;
                    '.' // replace the guard with an empty space
                },
                '>' => {
                    guard_position = (x as i32, y as i32);
                    guard_direction = E;
                    '.' // replace the guard with an empty space
                },
                _ => c
            }
        }).collect();
        grid.push(row);
    }
    (grid, guard_position, guard_direction)
}

fn move_guard(grid: &Grid, guard_position: &Position, guard_direction: &Direction) -> (Position, bool) {
    let (x, y) = guard_position;
    let (dx, dy) = guard_direction;
    let new_position = (x + dx, y + dy);
    let done = new_position.0 < 0 || new_position.0 >= grid[0].len() as i32 || new_position.1 < 0 || new_position.1 >= grid.len() as i32;
    (new_position, done)
}

fn turn_guard(guard_direction: &Direction) -> Direction {
    match *guard_direction {
        N => E,
        E => S,
        S => W,
        W => N,
        _ => N
    }
}

fn track_guard_path(grid: &mut Grid, guard_position: &Position, guard_direction: &Direction) -> i32 {
    let mut guard_position = *guard_position;
    let mut guard_direction = *guard_direction;
    let mut count = 1;
    let mut done = false;
    let mut new_position;
    grid[guard_position.1 as usize][guard_position.0 as usize] = 'X';
    while !done {
        (new_position, done) = move_guard(grid, &guard_position, &guard_direction);
        if !done {
            if grid[new_position.1 as usize][new_position.0 as usize] == '#' {
                guard_direction = turn_guard(&guard_direction);
            } else {
                guard_position = new_position;
                if grid[guard_position.1 as usize][guard_position.0 as usize] != 'X' {
                    grid[guard_position.1 as usize][guard_position.0 as usize] = 'X';
                    count += 1;
                }
            }
        }
    }
    count
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
