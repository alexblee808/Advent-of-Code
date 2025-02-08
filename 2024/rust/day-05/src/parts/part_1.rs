use log::debug;
use log::error;
use log::info;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_1(filename: &String) -> i32 {
    println!("Advent of Code 2024, Day 5 [Part 1]");

    // Create a file reader for the input file
    info!("Reading report data from file: {}", filename);
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    // Retrieve all the lines of text
    debug!("Reading lines of memory from file");
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut produce: bool = false;

    let memory_sums: Vec<i32> = reader
        .lines()
        .map(|line| match line {
            Ok(line) => {
                if line == "" {
                    produce = true;
                    info!("List of rules: {:#?}", rules);
                    0
                } else if produce == false {
                    let (x, y) = get_rule(line);
                    rules
                        .entry(x)
                        .and_modify(|x_y| x_y.push(y))
                        .or_insert(vec![y; 1]);
                    0
                } else if produce == true {
                    let pages: Vec<i32> =
                        line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

                    retrieve_page(&rules, pages)
                } else {
                    0
                }
            }
            Err(error) => {
                error!("Problem opening the file: {:?}", error);
                0
            }
        })
        .collect();

    let ans = memory_sums.iter().sum();

    // let ans: i32 = count_xmas(characters);
    println!("The number of XMAS occurrences is: {}", ans);
    return ans;
}

fn get_rule(rule: String) -> (i32, i32) {
    debug!("Parsing: {}", rule);

    let re: Regex = Regex::new(r"(?<x>[0-9]{1,3})\|(?<y>[0-9]{1,3})").unwrap();
    let Some(capture) = re.captures(&rule) else {
        println!("no match!");
        return (0, 0);
    };
    let x = capture.name("x").unwrap().as_str().parse::<i32>().unwrap();
    let y = capture.name("y").unwrap().as_str().parse::<i32>().unwrap();

    debug!("Parsed: {}|{}", x, y);
    return (x, y);
}

fn retrieve_page(rules: &HashMap<i32, Vec<i32>>, pages: Vec<i32>) -> i32 {
    debug!("Retrieving the middle page for: {:?}", pages);
    for (i, page) in pages.iter().enumerate() {
        if let Some(rule) = rules.get(page) {
            for prev_page in &pages[0..i] {
                if rule.contains(prev_page) {
                    debug!("Rule broken for: {}|{}", page, prev_page);
                    return 0;
                }
            }
        }
    }
    debug!("The middle page is: {}", pages[pages.len() / 2]);
    return pages[pages.len() / 2];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let result: i32 = part_1(&"inputs/test.txt".to_owned());
        assert_eq!(result, 143);
    }
}
