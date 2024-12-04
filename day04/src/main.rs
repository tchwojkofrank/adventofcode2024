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
    let xmas_map = get_xmas_map(contents);
    let xmas_count = count_xmas_in_map(&xmas_map);
    xmas_count.to_string()
}

fn get_xmas_map(contents: &String) -> Vec<Vec<char>> {
    let mut xmas_map: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        let mut xmas_line: Vec<char> = Vec::new();
        for c in line.chars() {
            xmas_line.push(c);
        }
        xmas_map.push(xmas_line);
    }
    xmas_map
}

fn count_xmas_in_map(xmas_map: &Vec<Vec<char>>) -> i32 {
    let mut xmas_count = 0;
    for y in 0..xmas_map.len() {
        for x in 0..xmas_map[y].len() {
            xmas_count += check_for_xmas_at_point(xmas_map, x, y);
        }
    }
    xmas_count
}

// count the number of instances of 'XMAS' starting at point (x,y) going in any direction
fn check_for_xmas_at_point(xmas_map: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut xmas_count = 0;
    // iterate through all the directions
    for dy in -1..2 {
        for dx in -1..2 {
            // if dx and dy are both zero, we are at the current point
            if dx == 0 && dy == 0 {
                continue;
            }
            // check for 'XMAS' starting at point (x,y) going in direction (dx,dy)
            if check_for_xmas_in_direction(xmas_map, (x,y), (dx, dy)) {
                xmas_count += 1;
            }
        }
    }

    xmas_count
}

// check for 'XMAS' starting at point (x,y) going in direction (dx,dy)
fn check_for_xmas_in_direction(xmas_map: &Vec<Vec<char>>, (x, y): (usize, usize), (dx, dy): (i32, i32)) -> bool {
    // if the direction is zero, we are not moving
    if dx == 0 && dy == 0 {
        return false;
    }
    // if the direction is not zero, we are moving
    let mut x = x as i32;
    let mut y = y as i32;
    let xmas = String::from("XMAS");
    // iterate through the characters in 'XMAS'
    for c in xmas.chars() {
        // if we are at the end of the map, we are done
        if x < 0 || y < 0 || x >= xmas_map[0].len() as i32 || y >= xmas_map.len() as i32 {
            return false;
        }
        // if the character at (x,y) is not the character in 'XMAS', we are done
        if xmas_map[y as usize][x as usize] != c {
            return false;
        }
        // move in the direction (dx,dy)
        x += dx;
        y += dy;
    }
    // if we made it through the loop, we found 'XMAS'
    true
}

#[allow(unused_variables)]
pub fn part2(contents: &String) -> String {
    let xmas_map = get_xmas_map(contents);
    let xmas_count = count_cross_mas_in_map(&xmas_map);
    xmas_count.to_string()
}

fn count_cross_mas_in_map(xmas_map: &Vec<Vec<char>>) -> i32 {
    let mut xmas_count = 0;
    for y in 0..xmas_map.len() {
        for x in 0..xmas_map[y].len() {
            if check_for_cross_mas_at_point(&xmas_map, x, y) {
                xmas_count += 1;
            }
        }
    }
    xmas_count
}

fn opposite_s_or_m(c: char) -> char {
    if c == 'S' {
        return 'M';
    }
    if c == 'M' {
        return 'S';
    }
    c
}

fn check_for_cross_mas_at_point(xmas_map: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    // make sure we're not at the edge of the map
    if x == 0 || y == 0 || x == xmas_map[0].len()-1 || y == xmas_map.len()-1 {
        return false;
    }
    if xmas_map[y][x] == 'A' {
        // check that both diagonals are 'MAS'
        // check down right diagonal
        if xmas_map[y+1][x+1] == 'S' || xmas_map[y+1][x+1] == 'M' {
            // check up left diagonal
            if xmas_map[y-1][x-1] == opposite_s_or_m(xmas_map[y+1][x+1]) {
                // check down left diagonal
                if xmas_map[y+1][x-1] == 'S' || xmas_map[y+1][x-1] == 'M' {
                    // check up right diagonal
                    if xmas_map[y-1][x+1] == opposite_s_or_m(xmas_map[y+1][x-1]) {
                        return true;
                    }
                }
            }
        }
    }
    false
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
