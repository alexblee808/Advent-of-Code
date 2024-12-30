use log::debug;
use log::error;
use log::info;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_2() {
    println!("Advent of Code 2024, Day 3 [Part 2]");

    // Create a file reader for the input file
    let filename = "input.txt";
    info!("Reading report data from file: {}", filename);
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    // Scan each report and determine if they are safe
    debug!("Reading lines of memory from file");
    let mut do_mul: bool = true;
    let memory_sums: Vec<i32> = reader
        .lines()
        .map(|line| match line {
            Ok(line) => {
                let (sum, _do_mul) = get_memory_sum(line, do_mul);
                do_mul = _do_mul;
                sum
            }
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

fn get_memory_sum(memory: String, do_mul: bool) -> (i32, bool) {
    let (products, _do_mul) = get_products(memory, do_mul);
    debug!("Taking the sum of the products: {:?}", products);
    let sum: i32 = products.iter().sum();
    debug!("The sum of the products is: {}", sum);
    return (sum, _do_mul);
}

fn get_products(memory: String, do_mul: bool) -> (Vec<i32>, bool) {
    debug!("Parsing the memory string: {}", memory);
    // Create a regex that matches on the union of all commands
    // Each command and argument is captured
    // Using the "extended mode" flag to write a nicer Regex
    let re = Regex::new(
        r#"(?x)
        (mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)) |
        (?<do>do\(\)) |
        (?<dont>don't\(\))
        "#,
    )
    .unwrap();

    let mut do_mul: bool = do_mul;
    let products: Vec<i32> = re
        .captures_iter(&memory)
        .map(|caps| {
            if caps.name("do") != None {
                do_mul = true;
                0
            } else if caps.name("dont") != None {
                do_mul = false;
                0
            } else if do_mul {
                // The unwraps are okay because every capture group must match if the whole
                // regex matches, and in this context, we know we have a match.
                //
                // Note that we use `caps.name("a").unwrap().as_str()` instead of
                // `&caps["a"]` because the lifetime of the former is the same as the
                // lifetime of `hay` above, but the lifetime of the latter is tied to the
                // lifetime of `caps` due to how the `Index` trait is defined.
                let mul_a = caps.name("a").unwrap().as_str().parse::<i32>().unwrap();
                let mul_b = caps.name("b").unwrap().as_str().parse::<i32>().unwrap();
                mul_a * mul_b
            } else {
                0
            }
        })
        .collect();

    return (products, do_mul);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_mul() {
        let result = get_memory_sum((&"mul(2,5)").to_string(), true);
        assert_eq!(result, (10, true));
    }
    #[test]
    fn multiple_mul() {
        let result = get_memory_sum((&"12nfmul(2,5)][2emul(1,4)23f").to_string(), true);
        assert_eq!(result, (14, true));
    }
    #[test]
    fn no_mul() {
        let result = get_memory_sum((&"nn3ml(2,5)3r31").to_string(), true);
        assert_eq!(result, (0, true));
    }
    #[test]
    fn do_mul() {
        let result = get_memory_sum((&"5vdo()nj244fmul(2,5)3v").to_string(), true);
        assert_eq!(result, (10, true));
    }
    #[test]
    fn dont_mul() {
        let result = get_memory_sum((&"23cdon't()23fmul(2,5)").to_string(), true);
        assert_eq!(result, (0, false));
    }
    #[test]
    fn double_dont_mul() {
        let result = get_memory_sum((&"23cdon't()23fdon't()23fmul(2,5)").to_string(), true);
        assert_eq!(result, (0, false));
    }
    #[test]
    fn example_mul() {
        let result = get_memory_sum(
            (&"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
                .to_string(),
            true,
        );
        assert_eq!(result, (48, true));
    }
}

// Incorrect submissions: 97977612 (too high)
