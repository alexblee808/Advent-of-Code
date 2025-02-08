use log::debug;
use log::error;
use log::info;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_1(filename: &String) -> i32 {
    println!("Advent of Code 2024, Day 4 [Part 1]");

    // Create a file reader for the input file
    info!("Reading report data from file: {}", filename);
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    // Retrieve all the lines of text
    debug!("Reading lines of memory from file");
    let characters: Vec<String> = reader
        .lines()
        .map(|line| match line {
            Ok(line) => line,
            Err(error) => {
                error!("Problem opening the file: {:?}", error);
                (&"").to_string()
            }
        })
        .collect();

    let ans: i32 = count_xmas(characters);
    println!("The number of XMAS occurrences is: {}", ans);
    return ans
}

fn count_xmas(characters: Vec<String>) -> i32 {
    let mut xmas_count: i32 = 0;

    for (c, row) in characters.iter().enumerate() {
        for (r, _) in row.chars().enumerate() {
            if characters[c].chars().nth(r).unwrap() != 'X' {
                continue;
            }
            for dc in [-1, 0, 1] {
                for dr in [-1, 0, 1] {
                    if dc == 0 && dr == 0 {
                        continue;
                    }
                    if r as i32 + 3 * dr < 0
                        || c as i32 + 3 * dc < 0
                        || r as i32 + 3 * dr > row.len() as i32 - 1
                        || c as i32 + 3 * dc > characters.len() as i32 - 1
                    {
                        continue;
                    }
                    if characters[(c as i32 + 1 * dc) as usize]
                        .chars()
                        .nth((r as i32 + 1 * dr) as usize)
                        .unwrap()
                        == 'M'
                        && characters[(c as i32 + 2 * dc) as usize]
                            .chars()
                            .nth((r as i32 + 2 * dr) as usize)
                            .unwrap()
                            == 'A'
                        && characters[(c as i32 + 3 * dc) as usize]
                            .chars()
                            .nth((r as i32 + 3 * dr) as usize)
                            .unwrap()
                            == 'S'
                    {
                        xmas_count += 1;
                    }
                }
            }
        }
    }
    return xmas_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let result: i32 = part_1(&"inputs/test.txt".to_owned());
        assert_eq!(result, 18);
    }
}
