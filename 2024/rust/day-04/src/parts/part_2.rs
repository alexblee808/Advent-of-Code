use log::debug;
use log::error;
use log::info;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_2(filename: &String) -> i32 {
    println!("Advent of Code 2024, Day 4 [Part 2]");

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
    println!("The number of X-MAS occurrences is: {}", ans);
    return ans
}

fn count_xmas(characters: Vec<String>) ->i32 {
    let mut xmas_count: i32 = 0;

    for (c, row) in characters.iter().enumerate() {
        for (r, _) in row.chars().enumerate() {
            // Add guard against boarder
            if c == 0 || r == 0 || c == characters.len() - 1 || r == row.len()-1 {continue;}
            if characters[c].chars().nth(r).unwrap() != 'A' {
                continue;
            }

            let neighbors: String = [characters[c-1].chars().nth(r-1).unwrap(),
            characters[c-1].chars().nth(r+1).unwrap(),
            characters[c+1].chars().nth(r+1).unwrap(),
            characters[c+1].chars().nth(r-1).unwrap()].iter().collect();

            if neighbors == "MMSS" || neighbors == "MSSM" || neighbors == "SSMM" || neighbors == "SMMS" {
                xmas_count += 1;
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
        let result: i32 = part_2(&"inputs/test.txt".to_owned());
        assert_eq!(result, 9);
    }
}
