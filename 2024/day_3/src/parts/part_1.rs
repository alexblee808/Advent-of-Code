use log::debug;
use log::info;
use log::error;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_1() {
    println!("Advent of Code 2024, Day 3 [Part 1]");

    // Create a file reader for the input file
    let filename = "input.txt";
    info!("Reading report data from file: {}", filename);
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    // Scan each report and determine if they are safe
    debug!("Reading lines of memory from file");
    let memory_sums: Vec<i32> = reader
        .lines()
        .map(|line| match line {
            Ok(line) => get_memory_sum(line),
            Err(error) => {
                error!("Problem opening the file: {:?}", error);
                0
            }
        })
        .collect();
    debug!("Read {} lines of memory", memory_sums.len());
    debug!("The memory sums are: {:?}", memory_sums);

    let total: i32 = memory_sums.iter().sum();
    println!(
        "The sum all of the results of the multiplications: {}",
        total
    );
}

fn get_memory_sum(memory: String) -> i32 {
    let products: Vec<i32> = get_products(memory);
    debug!("Taking the sum of the products: {:?}", products);
    let sum: i32 = products.iter().sum();
    debug!("The sum of the products is: {}", sum);
    return sum;
}

fn get_products(memory: String) -> Vec<i32> {
    debug!("Parsing the memory string: {}", memory);
    let re: Regex = Regex::new(r"mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)").unwrap();
    // 'm' is a 'Match', and 'as_str()' returns the matching part of the memory.
    let products: Vec<i32> = re
        .captures_iter(&memory)
        .map(|caps| {
            // The unwraps are okay because every capture group must match if the whole
            // regex matches, and in this context, we know we have a match.
            //
            // Note that we use `caps.name("a").unwrap().as_str()` instead of
            // `&caps["a"]` because the lifetime of the former is the same as the
            // lifetime of `hay` above, but the lifetime of the latter is tied to the
            // lifetime of `caps` due to how the `Index` trait is defined.
            let mul_a = caps.name("a").unwrap().as_str().parse::<i32>().unwrap();
            let mul_b = caps.name("b").unwrap().as_str().parse::<i32>().unwrap();

            debug!("Parsed: mul({},{})", mul_a, mul_b);
            mul_a * mul_b
        })
        .collect();

    return products;
}
