use log::debug;
use log::info;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_1() {
    println!("Advent of Code 2024, Day 1 [Part 1]");
    
    // Create a file reader for the input file
    let filename = "input.csv";
    info!("Reading distances from file: {}", filename);
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    // Read the locations and compile a list of all the distances
    debug!("Reading locations from file");
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let collection: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        left.push(collection[0]);
        right.push(collection[1]);
    }

    // Sort the locations from least to greatest
    debug!("Sorting locations");
    left.sort();
    right.sort();

    // Obtain the difference between the two lists
    debug!("Calculating differences");
    let mut dist: i64 = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        dist += (l - r).abs();
    }
    println!("The distance between the two lists is: {}", dist);
    
}
