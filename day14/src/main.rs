use std::time::Instant;
use regex::Regex;

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
    let result1 = part1(&contents,(101,103));
    let duration = start.elapsed();
    println!("Part 1:\n{}\n\tTook {:?}", result1, duration);

    let start = Instant::now();
    let result2 = part2(&contents,(101,103));
    let duration = start.elapsed();
    println!("Part 2:\n{}\n\tTook {:?}", result2, duration);
}

// turn off warning for unused variables
#[allow(unused_variables)]
pub fn part1(contents: &String,space: (i32,i32)) -> String {
    let mut robots = contents_to_robots(contents);
    for robot in robots.iter_mut() {
        let delta = (robot.v.0*100, robot.v.1*100);
        robot.p = (robot.p.0+delta.0, robot.p.1+delta.1);
        if robot.p.0 < 0 {
            robot.p.0 += space.0*(robot.p.0.abs()/space.0+1);
        }
        if robot.p.1 < 0 {
            robot.p.1 += space.1*(robot.p.1.abs()/space.1+1);
        }
        robot.p.0 %= space.0;
        robot.p.1 %= space.1;
    }
    let sf = safety_factor(&robots, space);
    sf.to_string()
}

struct Robot {
    p: (i32,i32),
    v: (i32,i32)
}

fn safety_factor(robots: &Vec<Robot>, space: (i32,i32)) -> i32 {
    let mut quadrants = vec![0, 0, 0, 0];
    // quadrants
    // 0 = top left
    // 1 = top right
    // 2 = bottom left
    // 3 = bottom right
    for robot in robots.iter() {
        if robot.p.0 < space.0/2 {
            if robot.p.1 < space.1/2 {
                quadrants[0] += 1;
            } else if robot.p.1 > space.1/2 {
                quadrants[2] += 1;
            }
        } else if robot.p.0 > space.0/2 {
            if robot.p.1 < space.1/2 {
                quadrants[1] += 1;
            } else if robot.p.1 > space.1/2 {
                quadrants[3] += 1;
            }
        }
    }
    quadrants.iter().product::<i32>()
}

fn contents_to_robots(contents: &String) -> Vec<Robot> {
    let mut robots = Vec::new();
    for line in contents.lines() {
        robots.push(line_to_robot(line));
    }
    robots
}

fn line_to_robot(line: &str) -> Robot {
    let mut robot = Robot { p: (0,0), v: (0,0) };
    // example input: p=0,4 v=3,-3
    // regex to capture the numbers
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let caps = re.captures(line).unwrap();
    robot.p.0 = caps[1].parse().unwrap();
    robot.p.1 = caps[2].parse().unwrap();
    robot.v.0 = caps[3].parse().unwrap();
    robot.v.1 = caps[4].parse().unwrap();
    robot
}

#[allow(unused_variables)]
pub fn part2(contents: &String,space: (i32,i32)) -> String {
    // wait for user input
    println!("Press enter to continue");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut robots = contents_to_robots(contents);
    for i in 0..10000000 {
        for robot in robots.iter_mut() {
            let delta = robot.v;
            robot.p = (robot.p.0+delta.0, robot.p.1+delta.1);
            if robot.p.0 < 0 {
                robot.p.0 += space.0*(robot.p.0.abs()/space.0+1);
            }
            if robot.p.1 < 0 {
                robot.p.1 += space.1*(robot.p.1.abs()/space.1+1);
            }
            robot.p.0 %= space.0;
            robot.p.1 %= space.1;
        }
        if i % 101 == 37{
            println!("{}", i);
            print_robots(&robots, space);
            println!();    
            // wait for 0.1s
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    1.to_string()
}

fn print_robots(robots: &Vec<Robot>, space: (i32,i32)) {
    let mut grid = vec![vec!['.'; space.0 as usize]; space.1 as usize];
    for robot in robots.iter() {
        grid[robot.p.1 as usize][robot.p.0 as usize] = '#';
    }
    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // get the contents of the file "files/test"
        let contents = advent::read_file_to_string("files/test");
        // call part1 with the contents of the file
        let result = part1(&contents,(11,7));
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
        let result = part2(&contents,(11,7));
        // get the contents of the file "files/test_answer_2"
        let answer = advent::read_file_to_string("files/test_answer_2");
        // compare the result with the answer
        assert_eq!(result, answer);
    }
}
