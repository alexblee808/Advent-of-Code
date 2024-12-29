use log::debug;
use log::info;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn part_2() {
    println!("Advent of Code 2024, Day 1 [Part 2]");
    
    // Create a file reader for the input file
    let filename = "input.csv";
    info!("Reading distances from file: {}", filename);
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    // Read the locations and compile a list of all the distances
    debug!("Reading locations from file");
    let mut left_location_count: HashMap<i32, i32> = HashMap::new();
    let mut right_location_count: HashMap<i32, i32> = HashMap::new();
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let collection: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let left_location: i32 = collection[0];
        let right_location: i32 = collection[1];

        match left_location_count.get(&left_location) {
            Some(count) => { left_location_count.insert(left_location, count + 1); }
            None => { left_location_count.insert(left_location, 1); }
        }
        match right_location_count.get(&right_location) {
            Some(count) => { right_location_count.insert(right_location, count + 1); }
            None => { right_location_count.insert(right_location, 1); }
        }
    }


    // Compute the similarity score
    debug!("Computing similarity");
    let mut similarity: i32 = 0;
    for (location, &left_count) in left_location_count.iter() {
        if let Some(right_count) = right_location_count.get(&location) {
            similarity += location*left_count*right_count;
        }
        
    }
    println!("The similarity between the two lists is: {}", similarity);
    
}
