#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let safe_reports: usize = input
        .lines()
        .map(|report| valid_report(report))
        .filter(|b| *b)
        .count();

    Ok(safe_reports.to_string())
}

fn valid_report(report: &str) -> bool {
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
    for (index, window) in collection.windows(2).enumerate() {
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
    return difference.abs() > 3 || difference == 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
