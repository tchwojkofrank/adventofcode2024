// use the advent package
use advent;
use std::collections::HashMap;

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
    let sections = contents.split("\n\n").collect::<Vec<&str>>();
    let rules = sections[0].lines().collect::<Vec<&str>>();
    let page_update_lines = sections[1].lines().collect::<Vec<&str>>();
    let rule_map = make_rule_map(&rules);
    let page_updates = make_page_updates(&page_update_lines);
    let result = process_page_updates(&rule_map, &page_updates);
    result.to_string()
}

fn process_page_updates(rule_map: &HashMap<i32, Vec<i32>>, page_updates: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for page_update in page_updates {
        result += is_valid_update(&rule_map, &page_update);
    }
    result
}

fn is_valid_update(rule_map: &HashMap<i32, Vec<i32>>, page_update: &Vec<i32>) -> i32 {
    // for each page in the update, check if all subsequent pages are in the rule_map for that page value
    for i in 0..page_update.len() - 1 {
        let page = page_update[i];
        let next_page = page_update[i + 1];
        if let Some(rule) = rule_map.get(&page) {
            if !rule.contains(&next_page) {
                return 0;
            }
        } else {
            return 0;
        }
    }
    // if we got this far, then the update is valid, and we return the middle value of the page update
    page_update[page_update.len() / 2]
}

fn make_rule_map(rules: &Vec<&str>) -> HashMap<i32, Vec<i32>> {
    let mut rule_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules {
        // split the rule into parts and convert the parts to integers
        let parts = rule.split("|").collect::<Vec<&str>>();
        let key = parts[0].parse::<i32>().unwrap();
        let value = parts[1].parse::<i32>().unwrap();
        // add the value to the vector in the hashmap
        rule_map.entry(key).or_insert(Vec::new()).push(value);
    }
    rule_map
}

fn make_page_updates(page_update_lines: &Vec<&str>) -> Vec<Vec<i32>> {
    let mut page_updates: Vec<Vec<i32>> = Vec::new();
    for line in page_update_lines {
        let page_update: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        page_updates.push(page_update);
    }
    page_updates
}

#[allow(unused_variables)]
pub fn part2(contents: &String) -> String {
    let sections = contents.split("\n\n").collect::<Vec<&str>>();
    let rules = sections[0].lines().collect::<Vec<&str>>();
    let page_update_lines = sections[1].lines().collect::<Vec<&str>>();
    let rule_map = make_rule_map(&rules);
    let page_updates = make_page_updates(&page_update_lines);
    let result = process_invalid_page_updates(&rule_map, &page_updates);
    result.to_string()
}

fn process_invalid_page_updates(rule_map: &HashMap<i32, Vec<i32>>, page_updates: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for page_update in page_updates {
        if is_valid_update(&rule_map, &page_update) == 0 {
            let fixed_page_update = fix_page_update(&rule_map, &page_update);
            result += fixed_page_update[fixed_page_update.len() / 2];
        }
    }
    result
}

// create a type alias called Page for an integer (i32)
type Page = i32;

fn fix_page_update(rule_map: &HashMap<i32, Vec<i32>>, page_update: &Vec<i32>) -> Vec<Page> {
    // create a Vector of Pages from the page_update
    let mut pages: Vec<Page> = page_update.iter().map(|x| *x).collect();
    // sort the pages using the rule_map to determine ordering
    // Page A is less than Page B if Page B is in the rule_map for Page A
    pages.sort_by(|a, b| {
        if let Some(rule) = rule_map.get(a) {
            if rule.contains(b) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        } else {
            std::cmp::Ordering::Greater
        }
    });

    pages
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
