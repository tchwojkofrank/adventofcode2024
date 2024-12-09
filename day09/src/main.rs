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
    let (block_ids, block_counts, space_counts) = block_parser(contents);
    let checksum = block_checksum(block_ids, &block_counts, &space_counts);
    checksum.to_string()
}

// returns the following:
// i32: number of block IDs.
// Vec<i32>: the repeat count for each block ID.
// Vec<i32>: the repeat count of space between each block ID.
// The input string alternates between one digit block repeat counts and one digit space repeat counts.
fn block_parser(block: &str) -> (i32, Vec<i32>, Vec<i32>) {
    let mut block_ids = 0;
    let mut block_counts = Vec::new();
    let mut space_counts = Vec::new();
    let mut block_count = true;
    for c in block.chars() {
        if block_count {
            block_ids += 1;
            block_counts.push(c.to_digit(10).unwrap() as i32);
        } else {
            space_counts.push(c.to_digit(10).unwrap() as i32);
        }
        block_count = !block_count;
    }
    (block_ids, block_counts, space_counts)
}

fn block_checksum(block_count: i32, block_counts: &Vec<i32>, space_counts: &Vec<i32>) -> u64 {
    let (block_list_size, block_list) = get_block_list(block_count, block_counts, space_counts);
    let mut checksum: u64 = 0;
    let mut end = block_list_size - 1;
    let mut compact_blocks = Vec::new();

    for i in 0..block_list_size {
        if i > end {
            break;
        }
        if block_list[i as usize] >= 0 {
            compact_blocks.push(block_list[i as usize]);
            print!("{}", block_list[i as usize]);
        } else {
            while block_list[end as usize] < 0 && end > i {
                end -= 1;
            }
            print!("{}", block_list[end as usize]);
            compact_blocks.push(block_list[end as usize]);
            end -= 1;
        }
    }
    for (i,v) in compact_blocks.iter().enumerate() {
        checksum += (*v as u64) * (i as u64);
    }
    println!();
    checksum
}

fn get_block_list(block_count: i32, block_counts: &Vec<i32>, space_counts: &Vec<i32>) -> (i32, Vec<i32>) {
    let mut block_list = Vec::new();
    let mut index = 0;
    for i in 0..block_count-1 {
        for _ in 0..block_counts[i as usize] {
            block_list.push(i);
            index += 1;
        }
        for _ in 0..space_counts[i as usize] {
            block_list.push(-1);
            index += 1;
        }
    }
    for _ in 0..block_counts[(block_count-1) as usize] {
        block_list.push(block_count-1);
        index += 1;
    }
    (index, block_list)
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
