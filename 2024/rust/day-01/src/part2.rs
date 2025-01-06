use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut left_location_count: HashMap<i32, i32> = HashMap::new();
    let mut right_location_count: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let collection: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        left_location_count
            .entry(collection[0])
            .and_modify(|count| *count += 1)
            .or_insert(1);

        right_location_count
            .entry(collection[1])
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut similarity: i32 = 0;
    for (location, &left_count) in left_location_count.iter() {
        if let Some(right_count) = right_location_count.get(location) {
            similarity += location * left_count * right_count;
        }
    }

    Ok(similarity.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
