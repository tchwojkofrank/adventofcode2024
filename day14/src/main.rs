use std::time::Instant;
use advent::*;
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
    let result1 = part1(&contents,Point2D { x: 101, y: 103 });
    let duration = start.elapsed();
    println!("Part 1:\n{}\n\tTook {:?}", result1, duration);

    let start = Instant::now();
    let result2 = part2(&contents,Point2D { x: 101, y: 103 });
    let duration = start.elapsed();
    println!("Part 2:\n{}\n\tTook {:?}", result2, duration);
}

// turn off warning for unused variables
#[allow(unused_variables)]
pub fn part1(contents: &String,space: Point2D<i32>) -> String {
    let mut robots = contents_to_robots(contents);
    for robot in robots.iter_mut() {
        let delta = robot.v.scale(100);
        robot.p = robot.p.add(delta);
        if robot.p.x < 0 {
            robot.p.x += space.x*(robot.p.x.abs()/space.x+1);
        }
        if robot.p.y < 0 {
            robot.p.y += space.y*(robot.p.y.abs()/space.y+1);
        }
        robot.p.x %= space.x;
        robot.p.y %= space.y;
    }
    let sf = safety_factor(&robots, space);
    sf.to_string()
}

struct Robot {
    p: Point2D<i32>,
    v: Point2D<i32>
}

fn safety_factor(robots: &Vec<Robot>, space: Point2D<i32>) -> i32 {
    let mut quadrants = vec![0, 0, 0, 0];
    // quadrants
    // 0 = top left
    // 1 = top right
    // 2 = bottom left
    // 3 = bottom right
    for robot in robots.iter() {
        if robot.p.x < space.x/2 {
            if robot.p.y < space.y/2 {
                quadrants[0] += 1;
            } else if robot.p.y > space.y/2 {
                quadrants[2] += 1;
            }
        } else if robot.p.x > space.x/2 {
            if robot.p.y < space.y/2 {
                quadrants[1] += 1;
            } else if robot.p.y > space.y/2 {
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
    let mut robot = Robot { p: Point2D { x: 0, y: 0 }, v: Point2D { x: 0, y: 0 } };
    // example input: p=0,4 v=3,-3
    // regex to capture the numbers
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let caps = re.captures(line).unwrap();
    robot.p.x = caps[1].parse().unwrap();
    robot.p.y = caps[2].parse().unwrap();
    robot.v.x = caps[3].parse().unwrap();
    robot.v.y = caps[4].parse().unwrap();
    robot
}

#[allow(unused_variables)]
pub fn part2(contents: &String,space: Point2D<i32>) -> String {
    let mut robots = contents_to_robots(contents);
    for i in 0..10000000 {
        for robot in robots.iter_mut() {
            let delta = robot.v;
            robot.p = robot.p.add(delta);
            if robot.p.x < 0 {
                robot.p.x += space.x*(robot.p.x.abs()/space.x+1);
            }
            if robot.p.y < 0 {
                robot.p.y += space.y*(robot.p.y.abs()/space.y+1);
            }
            robot.p.x %= space.x;
            robot.p.y %= space.y;
        }
        if i % 101 == 37{
            println!("{}", i);
            print_robots(&robots, space);
            println!();    
            // wait for user input
            // let mut input = String::new();
            // std::io::stdin().read_line(&mut input).unwrap();
            // wait for 0.1s
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    1.to_string()
}

fn print_robots(robots: &Vec<Robot>, space: Point2D<i32>) {
    let mut grid = vec![vec!['.'; space.x as usize]; space.y as usize];
    for robot in robots.iter() {
        grid[robot.p.y as usize][robot.p.x as usize] = '#';
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
        let result = part1(&contents,Point2D { x: 11, y: 7 });
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
        let result = part2(&contents,Point2D { x: 11, y: 7 });
        // get the contents of the file "files/test_answer_2"
        let answer = advent::read_file_to_string("files/test_answer_2");
        // compare the result with the answer
        assert_eq!(result, answer);
    }
}
