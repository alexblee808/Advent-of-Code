use log::debug;
use log::error;
use log::info;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_2(filename: &String) -> i32 {
    println!("Advent of Code 2024, Day 6 [Part 2]");

    // Create a file reader for the input file
    info!("Reading report data from file: {}", filename);
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    // Retrieve the starting map
    debug!("Reading map from file");
    let mut map: Vec<String> = reader
        .lines()
        .map(|line| match line {
            Ok(line) => line,
            Err(error) => {
                error!("Problem opening the file: {:?}", error);
                "".to_string()
            }
        })
        .collect();

    let original_map = map.clone();
    let mut positions: HashSet<(i32, i32)> = HashSet::new(); // Store list of each obstacle resulting in a loop
    let mut loc: (i32, i32) = locate_guard(&map);
    let height: i32 = map.len() as i32;
    let width: i32 = map[0].len() as i32;

    // Iterate over the map until the guard leaves, getting all possible positions for the obstacle
    while loc.0 > 0 && loc.1 > 0 && loc.0 < height && loc.1 < width {
        let orientation = map[loc.0 as usize].chars().nth(loc.1 as usize).unwrap();
        let tmp_loc: (i32, i32) = loc;

        let (next_loc, adj_loc, adj_orientation) = match &orientation {
            '^' => ((loc.0 - 1, loc.1), (loc.0, loc.1 + 1), &">"),
            '>' => ((loc.0, loc.1 + 1), (loc.0 + 1, loc.1), &"v"),
            'v' => ((loc.0 + 1, loc.1), (loc.0, loc.1 - 1), &"<"),
            '<' => ((loc.0, loc.1 - 1), (loc.0 - 1, loc.1), &"^"),
            _ => {
                error!("Did not find guard at {:?}", loc);
                debug!("Errant map:\n{:?}", map);
                ((-1, -1), (-1, -1), &" ")
            }
        };

        // Guard clause if guard leaves the map
        if next_loc.0 < 0 || next_loc.1 < 0 || next_loc.0 > height - 1 || next_loc.1 > width - 1 {
            break;
        }

        // Update the map and the guard's location
        if map[next_loc.0 as usize]
            .chars()
            .nth(next_loc.1 as usize)
            .unwrap()
            .eq(&'#')
        {
            loc = adj_loc;
            if map[loc.0 as usize].chars().nth(loc.1 as usize).unwrap() != 'X' {
                positions.insert(loc);
            }
            map[loc.0 as usize].replace_range(loc.1 as usize..loc.1 as usize + 1, adj_orientation);
        } else {
            loc = next_loc;
            if map[loc.0 as usize].chars().nth(loc.1 as usize).unwrap() != 'X' {
                positions.insert(loc);
            }
            map[loc.0 as usize].replace_range(
                loc.1 as usize..loc.1 as usize + 1,
                orientation.to_string().as_str(),
            );
        }
        map[tmp_loc.0 as usize].replace_range(tmp_loc.1 as usize..tmp_loc.1 as usize + 1, &"X");
    }

    let ans: usize = positions
        .iter()
        .map(|obstacle| obstacle_loop(original_map.clone(), *obstacle))
        .filter(|x| *x)
        .count(); // Number of single-obstruction loops
    println!("The number of single obstructions loops is: {}", ans);
    return ans as i32;
}

fn locate_guard(map: &Vec<String>) -> (i32, i32) {
    // Determine the location of the guard given a map
    for (r, row) in map.iter().enumerate() {
        for (c, position) in row.chars().enumerate() {
            if "^<>v".contains(position) {
                return (r as i32, c as i32);
            }
        }
    }
    return (-1, -1);
}

fn obstacle_loop(mut map: Vec<String>, obstacle: (i32, i32)) -> bool {
    // Determines if placing an obstacle in the next position would result in a loop
    let mut loc: (i32, i32) = locate_guard(&map);

    let height: i32 = map.len() as i32;
    let width: i32 = map[0].len() as i32;

    let mut steps: HashSet<(i32, i32, char)> = HashSet::new();
    // Iterate over the map until the guard leaves
    while loc.0 > 0 && loc.1 > 0 && loc.0 < height && loc.1 < width {
        let orientation = map[loc.0 as usize].chars().nth(loc.1 as usize).unwrap();
        if steps.contains(&(loc.0, loc.1, orientation)) {
            return true;
        } else {
            steps.insert((loc.0, loc.1, orientation));
        }

        let tmp_loc: (i32, i32) = loc;

        let (next_loc, adj_loc, adj_orientation) = match &orientation {
            '^' => ((loc.0 - 1, loc.1), (loc.0, loc.1 + 1), &">"),
            '>' => ((loc.0, loc.1 + 1), (loc.0 + 1, loc.1), &"v"),
            'v' => ((loc.0 + 1, loc.1), (loc.0, loc.1 - 1), &"<"),
            '<' => ((loc.0, loc.1 - 1), (loc.0 - 1, loc.1), &"^"),
            _ => {
                error!("Did not find guard at {:?}", loc);
                debug!("Errant map:\n{:?}", map);
                ((-1, -1), (-1, -1), &" ")
            }
        };

        // Guard clause if guard leaves the map
        if next_loc.0 < 0 || next_loc.1 < 0 || next_loc.0 > height - 1 || next_loc.1 > width - 1 {
            break;
        }

        // Update the map and the guard's location
        if map[next_loc.0 as usize]
            .chars()
            .nth(next_loc.1 as usize)
            .unwrap()
            .eq(&'#')
            || next_loc == obstacle
        {
            loc = adj_loc;

            map[loc.0 as usize].replace_range(loc.1 as usize..loc.1 as usize + 1, adj_orientation);
        } else {
            loc = next_loc;

            map[loc.0 as usize].replace_range(
                loc.1 as usize..loc.1 as usize + 1,
                orientation.to_string().as_str(),
            );
        }
        map[tmp_loc.0 as usize].replace_range(tmp_loc.1 as usize..tmp_loc.1 as usize + 1, &"X");
    }
    return false;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let result: i32 = part_2(&"inputs/test.txt".to_owned());
        assert_eq!(result, 6);
    }
}

// 1577: too low
