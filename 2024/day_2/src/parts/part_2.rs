use log::debug;
use log::info;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_2() {
    println!("Advent of Code 2024, Day 2 [Part 2]");

    // Create a file reader for the input file
    let filename = "input.csv";
    info!("Reading report data from file: {}", filename);
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    // Scan each report and determine if they are safe
    debug!("Reading reports from file");
    let mut safe_reports: i32 = 0;
    // Each `line` is a report
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        
        if valid_report(line) {
            safe_reports += 1
        }
    }
    println!("Safe reports: {}", safe_reports);
}

fn valid_report(report: String) -> bool {
    // Split the report into a vector of levels
    let collection: Vec<i32> = report
    .split_whitespace()
    .map(|x| x.parse::<i32>().unwrap())
    .collect();

    if validate_report(collection.clone()) {
        return true;
    }

    for i in 0..(collection.len()) {
        let mut new_report = collection.clone();
        new_report.remove(i);

        if validate_report(new_report) {
            return true;
        }
    }

    return false;
}

fn validate_report(collection: Vec<i32>) -> bool {
    let mut increasing: bool = true; // Set a default value to overwrite

    // Process each pair of levels with a third backup in the event of unsafety
    for (index,window ) in collection.windows(2).enumerate() {

        // If the first window pair breaks the rule, reprocess on the second pair
        let level_a = window[0];
        let level_b = window[1];

        if invalid_level_difference(level_a, level_b) {
            return false;
        }

        // Must set the increasing state after it is determined whether or not the
        // inputted pair is valid
        if index.eq(&0) {
            increasing = level_b > level_a;
        }

        if increasing != (level_b > level_a) {
            return false;
        }
    }
    return true;
}

fn invalid_level_difference(level_a: i32, level_b: i32) -> bool {
    // Check if the adjacent level difference is within the threshold
    let difference: i32 = level_b - level_a;
    return difference.abs() > 3  || difference == 0 
}

// Incorrect results: 612, 680, 802